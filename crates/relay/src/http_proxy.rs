use anyhow::Result;
use dashmap::DashMap;
use http_body_util::combinators::BoxBody;
use http_body_util::BodyExt;
use hyper::body::{Bytes, Incoming};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use std::net::IpAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tracing::{error, info, warn};
use uuid::Uuid;

pub async fn start_http_proxy(
    bind: IpAddr,
    http_clients: Arc<DashMap<String, tokio::sync::mpsc::Sender<Uuid>>>,
    http_tunnels: Arc<DashMap<Uuid, tokio::sync::oneshot::Sender<TcpStream>>>,
) -> Result<()> {
    let port: u16 = std::env::var("HTTP_PORT").unwrap_or_else(|_| "80".to_string()).parse().unwrap_or(80);
    let listener = TcpListener::bind((bind, port)).await?;
    info!(addr = ?bind, port, "HTTP proxy listening");

    loop {
        let (stream, _addr) = match listener.accept().await {
            Ok(res) => res,
            Err(err) => {
                warn!(%err, "Failed to accept HTTP connection");
                continue;
            }
        };

        let io = TokioIo::new(stream);
        let http_clients = http_clients.clone();
        let http_tunnels = http_tunnels.clone();

        tokio::spawn(async move {
            let service = service_fn(move |req| {
                let http_clients = http_clients.clone();
                let http_tunnels = http_tunnels.clone();
                async move {
                    handle_request(req, http_clients, http_tunnels).await
                }
            });

            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service)
                .await
            {
                error!(%err, "Error serving HTTP connection");
            }
        });
    }
}

async fn handle_request(
    req: Request<Incoming>,
    http_clients: Arc<DashMap<String, tokio::sync::mpsc::Sender<Uuid>>>,
    http_tunnels: Arc<DashMap<Uuid, tokio::sync::oneshot::Sender<TcpStream>>>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let host = match req.headers().get(hyper::header::HOST) {
        Some(host) => host.to_str().unwrap_or(""),
        None => "",
    };

    let domain = host.split(':').next().unwrap_or("").to_string();

    let client_tx = match http_clients.get(&domain) {
        Some(tx) => tx.clone(),
        None => {
            return Ok(not_found());
        }
    };

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

    let res = match sender.send_request(req).await {
        Ok(res) => res,
        Err(err) => {
            error!(%err, "Failed to send request");
            return Ok(internal_server_error("Failed to proxy request"));
        }
    };

    Ok(res.map(|b| b.boxed()))
}

fn not_found() -> Response<BoxBody<Bytes, hyper::Error>> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(
            http_body_util::Full::new(Bytes::from("404 Not Found: Subdomain not registered"))
                .map_err(|e| match e {})
                .boxed(),
        )
        .unwrap()
}

fn internal_server_error(msg: &'static str) -> Response<BoxBody<Bytes, hyper::Error>> {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(
            http_body_util::Full::new(Bytes::from(msg))
                .map_err(|e| match e {})
                .boxed(),
        )
        .unwrap()
}
