use anyhow::Result;
use base64::Engine;
use dashmap::DashMap;
use http_body_util::BodyExt;
use http_body_util::combinators::BoxBody;
use hyper::body::{Bytes, Incoming};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use relay_common::model::http::AuthConfig;
use std::future::Future;
use std::net::IpAddr;
use std::sync::Arc;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::{TcpListener, TcpStream};
use tokio_rustls::LazyConfigAcceptor;
use tracing::{error, info, warn};
use uuid::Uuid;

pub type HttpClientMap =
    Arc<DashMap<String, (tokio::sync::mpsc::Sender<Uuid>, Option<AuthConfig>)>>;
pub type HttpTunnelMap = Arc<DashMap<Uuid, tokio::sync::oneshot::Sender<TcpStream>>>;

fn env_port(var: &str, default: u16) -> u16 {
    std::env::var(var)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

async fn run_proxy<S, F, Fut>(
    listener: TcpListener,
    http_clients: HttpClientMap,
    http_tunnels: HttpTunnelMap,
    prepare: F,
) where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    F: Fn(TcpStream) -> Fut + Send + Clone + 'static,
    Fut: Future<Output = Option<S>> + Send,
{
    loop {
        let (stream, _addr) = match listener.accept().await {
            Ok(res) => res,
            Err(err) => {
                warn!(%err, "Failed to accept connection");
                continue;
            }
        };

        let http_clients = http_clients.clone();
        let http_tunnels = http_tunnels.clone();
        let prepare = prepare.clone();

        tokio::spawn(async move {
            let Some(stream) = prepare(stream).await else {
                return;
            };

            let io = TokioIo::new(stream);
            let service = service_fn(move |req| {
                let http_clients = http_clients.clone();
                let http_tunnels = http_tunnels.clone();
                async move { handle_request(req, http_clients, http_tunnels).await }
            });

            if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                error!(%err, "Error serving connection");
            }
        });
    }
}

pub async fn start_http_proxy(
    bind: IpAddr,
    http_clients: HttpClientMap,
    http_tunnels: HttpTunnelMap,
) -> Result<()> {
    let port = env_port("HTTP_PORT", 80);
    let listener = TcpListener::bind((bind, port)).await?;
    info!(addr = ?bind, port, "HTTP proxy listening");

    run_proxy(listener, http_clients, http_tunnels, |stream| async move {
        Some(stream)
    })
    .await;

    Ok(())
}

pub async fn start_https_proxy(
    bind: IpAddr,
    http_clients: HttpClientMap,
    http_tunnels: HttpTunnelMap,
) -> Result<()> {
    let port = env_port("HTTPS_PORT", 443);
    let listener = TcpListener::bind((bind, port)).await?;
    info!(addr = ?bind, port, "HTTPS proxy listening");

    let subject_alt_names = vec![
        "*.relay.invalidjoker.dev".to_string(),
        "relay.invalidjoker.dev".to_string(),
    ];
    let cert = rcgen::generate_simple_self_signed(subject_alt_names)?;
    let cert_der = cert.cert.der().to_vec();
    let priv_key_der = cert.signing_key.serialize_der();

    let priv_key = rustls_pki_types::PrivateKeyDer::try_from(priv_key_der)
        .map_err(|e| anyhow::anyhow!("Invalid private key: {}", e))?;
    let cert_chain = vec![rustls_pki_types::CertificateDer::from(cert_der).into_owned()];

    let server_config = Arc::new(
        rustls::ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(cert_chain, priv_key)?,
    );

    run_proxy(listener, http_clients, http_tunnels, move |stream| {
        let server_config = server_config.clone();
        async move {
            let acceptor = LazyConfigAcceptor::new(rustls::server::Acceptor::default(), stream);
            let start = match acceptor.await {
                Ok(start) => start,
                Err(err) => {
                    warn!(%err, "Failed to read ClientHello");
                    return None;
                }
            };

            match start.into_stream(server_config).await {
                Ok(stream) => Some(stream),
                Err(err) => {
                    warn!(%err, "TLS handshake failed");
                    None
                }
            }
        }
    })
    .await;

    Ok(())
}

fn is_authorized(req: &Request<Incoming>, auth: &AuthConfig) -> bool {
    let Some(header) = req
        .headers()
        .get(hyper::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
    else {
        return false;
    };

    let Some(token) = header.strip_prefix("Basic ") else {
        return false;
    };

    let Ok(decoded) = base64::prelude::BASE64_STANDARD.decode(token) else {
        return false;
    };

    let Ok(decoded) = String::from_utf8(decoded) else {
        return false;
    };

    let Some((user, pass)) = decoded.split_once(':') else {
        return false;
    };

    user == auth.username && pass == auth.password
}

async fn handle_request(
    req: Request<Incoming>,
    http_clients: HttpClientMap,
    http_tunnels: HttpTunnelMap,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let host = req
        .headers()
        .get(hyper::header::HOST)
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");
    let domain = host.split(':').next().unwrap_or("").to_string();

    let Some(client_entry) = http_clients.get(&domain).map(|e| e.clone()) else {
        return Ok(not_found());
    };
    let (client_tx, auth_config) = client_entry;

    if let Some(auth) = auth_config
        && !is_authorized(&req, &auth)
    {
        return Ok(unauthorized());
    }

    let id = Uuid::new_v4();
    let (tx, rx) = tokio::sync::oneshot::channel();
    http_tunnels.insert(id, tx);

    if client_tx.send(id).await.is_err() {
        http_tunnels.remove(&id);
        return Ok(internal_server_error("Client disconnected"));
    }

    let tunnel_stream = match tokio::time::timeout(std::time::Duration::from_secs(10), rx).await {
        Ok(Ok(stream)) => stream,
        _ => {
            http_tunnels.remove(&id);
            return Ok(internal_server_error("Tunnel timeout"));
        }
    };

    let io = TokioIo::new(tunnel_stream);

    let (mut sender, conn) = match hyper::client::conn::http1::Builder::new()
        .handshake(io)
        .await
    {
        Ok(res) => res,
        Err(err) => {
            error!(%err, "Client handshake failed");
            return Ok(internal_server_error("Handshake failed"));
        }
    };

    tokio::spawn(async move {
        if let Err(err) = conn.await {
            error!(%err, "Connection failed");
        }
    });

    match sender.send_request(req).await {
        Ok(res) => Ok(res.map(|b| b.boxed())),
        Err(err) => {
            error!(%err, "Failed to send request");
            Ok(internal_server_error("Failed to proxy request"))
        }
    }
}

fn body_from(msg: impl Into<Bytes>) -> BoxBody<Bytes, hyper::Error> {
    http_body_util::Full::new(msg.into())
        .map_err(|e| match e {})
        .boxed()
}

fn not_found() -> Response<BoxBody<Bytes, hyper::Error>> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(body_from("404 Not Found: Subdomain not registered"))
        .unwrap()
}

fn unauthorized() -> Response<BoxBody<Bytes, hyper::Error>> {
    Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .header(
            hyper::header::WWW_AUTHENTICATE,
            "Basic realm=\"Restricted\"",
        )
        .body(body_from("401 Unauthorized: Invalid credentials"))
        .unwrap()
}

fn internal_server_error(msg: &'static str) -> Response<BoxBody<Bytes, hyper::Error>> {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(body_from(msg))
        .unwrap()
}
