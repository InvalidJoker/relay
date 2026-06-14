use crate::auth::Authentication;
use anyhow::Result;
use dashmap::DashMap;
use relay_common::connection::{ClientMessage, RelayMessage};
use relay_common::constants::RELAY_PORT;
use relay_common::model::relay::HostConfig;
use relay_common::worker::StreamWorker;
use std::net::IpAddr;
use std::ops::RangeInclusive;
use std::sync::Arc;
use std::time::Duration;
use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{sleep, timeout};
use tracing::{Instrument, info, info_span, warn};
use uuid::Uuid;

pub struct Server {
    /// The port range to listen on.
    port_range: RangeInclusive<u16>,

    /// The address to bind everything to
    bind: IpAddr,

    auth: Authentication,

    /// Public URL
    pub_url: String,

    /// Concurrent map of IDs to incoming connections.
    tcp_conns: Arc<DashMap<Uuid, TcpStream>>,

    http_tunnels: Arc<DashMap<Uuid, tokio::sync::oneshot::Sender<TcpStream>>>,
    http_clients: Arc<
        DashMap<
            String,
            (
                tokio::sync::mpsc::Sender<Uuid>,
                Option<relay_common::model::relay::HttpAuthConfig>,
            ),
        >,
    >,
}

impl Server {
    pub fn new(port_range: RangeInclusive<u16>, bind: IpAddr, backend_url: String) -> Self {
        Self {
            port_range,
            bind,
            auth: Authentication::new(backend_url),
            tcp_conns: Arc::new(DashMap::new()),
            http_tunnels: Arc::new(DashMap::new()),
            http_clients: Arc::new(DashMap::new()),
            pub_url: std::env::var("PUBLIC_URL")
                .unwrap_or_else(|_| "relay.invalidjoker.dev".to_string()),
        }
    }

    pub async fn listen(self) -> Result<()> {
        let this = Arc::new(self);

        let hc = Arc::clone(&this.http_clients);
        let ht = Arc::clone(&this.http_tunnels);
        let bind = this.bind;
        tokio::spawn(async move {
            if let Err(err) = crate::http_proxy::start_http_proxy(bind, hc, ht).await {
                warn!(%err, "HTTP proxy exited with error");
            }
        });

        let hc2 = Arc::clone(&this.http_clients);
        let ht2 = Arc::clone(&this.http_tunnels);
        tokio::spawn(async move {
            if let Err(err) = crate::http_proxy::start_https_proxy(bind, hc2, ht2).await {
                warn!(%err, "HTTPS proxy exited with error");
            }
        });

        let listener = TcpListener::bind((this.bind, RELAY_PORT)).await?;
        info!(addr = ?this.bind, "server listening");

        loop {
            let (stream, addr) = listener.accept().await?;
            let this = Arc::clone(&this);
            tokio::spawn(
                async move {
                    info!("incoming connection");
                    if let Err(err) = this.handle_connection(stream).await {
                        warn!(%err, "connection exited with error");
                    } else {
                        info!("connection exited");
                    }
                }
                .instrument(info_span!("control", ?addr)),
            );
        }
    }

    async fn create_listener(&self, port: Option<u16>) -> Result<TcpListener, &'static str> {
        let try_bind = |port: u16| async move {
            TcpListener::bind((self.bind, port))
                .await
                .map_err(|err| match err.kind() {
                    io::ErrorKind::AddrInUse => "port already in use",
                    io::ErrorKind::PermissionDenied => "permission denied",
                    _ => "failed to bind to port",
                })
        };
        if let Some(port) = port {
            if !self.port_range.contains(&port) {
                return Err("client port number not in allowed range");
            }
            try_bind(port).await
        } else {
            let base = *self.port_range.start();
            let len = self.port_range.clone().count();

            let start = fastrand::usize(..len);

            for i in 0..len {
                let port = base + ((start + i) % len) as u16;

                if let Ok(listener) = try_bind(port).await {
                    return Ok(listener);
                }
            }

            Err("failed to find an available port")
        }
    }

    async fn handle_connection(&self, stream: TcpStream) -> Result<()> {
        let mut stream = StreamWorker::new(stream);

        match stream.recv_timeout().await? {
            Some(ClientMessage::Hello(msg)) => {
                match msg.host_config {
                    HostConfig::Tcp(tcp) => {
                        if let Err(err) = self.auth.check_tcp(&msg.token, tcp.remote_port).await {
                            warn!(%err, "authentication failed");
                            stream
                                .send(RelayMessage::Error("authentication failed".to_string()))
                                .await?;
                            return Ok(());
                        }

                        let listener = match self.create_listener(tcp.remote_port).await {
                            Ok(listener) => listener,
                            Err(err) => {
                                stream.send(RelayMessage::Error(err.into())).await?;
                                return Ok(());
                            }
                        };
                        let host = listener.local_addr()?.ip();
                        let port = listener.local_addr()?.port();
                        info!(?host, ?port, "new client");
                        stream.send(RelayMessage::Hello(port.to_string())).await?;

                        loop {
                            if stream.send(RelayMessage::Heartbeat).await.is_err() {
                                // Assume that the TCP connection has been dropped.
                                return Ok(());
                            }
                            const TIMEOUT: Duration = Duration::from_millis(500);
                            if let Ok(result) = timeout(TIMEOUT, listener.accept()).await {
                                let (stream2, addr) = result?;
                                info!(?addr, ?port, "new connection");

                                let id = Uuid::new_v4();
                                let conns = Arc::clone(&self.tcp_conns);

                                conns.insert(id, stream2);
                                tokio::spawn(async move {
                                    sleep(Duration::from_secs(10)).await;
                                    if conns.remove(&id).is_some() {
                                        warn!(%id, "removed stale connection");
                                    }
                                });
                                stream.send(RelayMessage::Connection(id)).await?;
                            }
                        }
                    }
                    HostConfig::Http(http) => {
                        if let Err(err) =
                            self.auth.check_http(&msg.token, http.domain.clone()).await
                        {
                            warn!(%err, "authentication failed");
                            stream
                                .send(RelayMessage::Error("authentication failed".to_string()))
                                .await?;
                            return Ok(());
                        }

                        let domain = match http.domain {
                            Some(d) => {
                                if d.contains('.') {
                                    d
                                } else {
                                    format!("{}.{}", d, self.pub_url)
                                }
                            }
                            None => {
                                let nato_alphabet = [
                                    "alpha", "bravo", "charlie", "delta", "echo", "foxtrot",
                                    "golf", "hotel", "india", "juliett", "kilo", "lima", "mike",
                                    "november", "oscar", "papa", "quebec", "romeo", "sierra",
                                    "tango", "uniform", "victor", "whiskey", "x-ray", "yankee",
                                    "zulu",
                                ];
                                let mut subdomain = String::new();
                                for _ in 0..3 {
                                    subdomain.push_str(
                                        nato_alphabet[fastrand::usize(..nato_alphabet.len())],
                                    );
                                }
                                format!("{}.{}", subdomain, self.pub_url)
                            }
                        };

                        info!(?domain, "new http client");
                        let (tx, mut rx) = tokio::sync::mpsc::channel(32);
                        self.http_clients.insert(domain.clone(), (tx, http.auth));
                        stream.send(RelayMessage::Hello(domain.clone())).await?;

                        loop {
                            tokio::select! {
                                _ = sleep(Duration::from_millis(500)) => {
                                    if stream.send(RelayMessage::Heartbeat).await.is_err() {
                                        self.http_clients.remove(&domain);
                                        return Ok(());
                                    }
                                }
                                id = rx.recv() => {
                                    match id {
                                        Some(id) => {
                                            if stream.send(RelayMessage::Connection(id)).await.is_err() {
                                                self.http_clients.remove(&domain);
                                                return Ok(());
                                            }
                                        }
                                        None => {
                                            self.http_clients.remove(&domain);
                                            return Ok(());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Some(ClientMessage::Accept(id)) => {
                info!(%id, "forwarding connection");

                if let Some((_, tx)) = self.http_tunnels.remove(&id) {
                    let parts = stream.into_parts();
                    if tx.send(parts.io).is_err() {
                        warn!(%id, "failed to send tunnel to http proxy");
                    }
                    return Ok(());
                }

                match self.tcp_conns.remove(&id) {
                    Some((_, mut stream2)) => {
                        let mut parts = stream.into_parts();
                        debug_assert!(parts.write_buf.is_empty(), "framed write buffer not empty");
                        stream2.write_all(&parts.read_buf).await?;
                        tokio::io::copy_bidirectional(&mut parts.io, &mut stream2).await?;
                    }
                    None => warn!(%id, "missing connection"),
                }
                Ok(())
            }
            None => Ok(()),
        }
    }
}
