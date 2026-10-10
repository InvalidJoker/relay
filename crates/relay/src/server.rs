use crate::auth::Authentication;
use crate::http_proxy::{HttpClient, HttpClientMap, HttpTunnelMap};
use anyhow::Result;
use dashmap::DashMap;
use dashmap::mapref::entry::Entry;
use relay_common::constants::RELAY_PORT;
use relay_common::messages::{ClientMessage, RelayMessage};
use relay_common::model::HostConfig;
use relay_common::model::http::HostConfig as HttpHostConfig;
use relay_common::model::tcp::HostConfig as TcpHostConfig;
use relay_common::worker::StreamWorker;
use std::net::IpAddr;
use std::ops::RangeInclusive;
use std::sync::Arc;
use std::time::Duration;
use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{MissedTickBehavior, interval, sleep};
use tracing::{Instrument, info, info_span, warn};
use uuid::Uuid;

/// How often the client is pinged so a dead connection is noticed.
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long an accepted connection waits for the client to pick it up.
const PENDING_CONNECTION_TTL: Duration = Duration::from_secs(10);

pub struct Server {
    /// The port range to listen on.
    port_range: RangeInclusive<u16>,

    /// The address to bind everything to
    bind: IpAddr,

    auth: Authentication,

    /// Concurrent map of IDs to incoming connections.
    tcp_conns: Arc<DashMap<Uuid, TcpStream>>,

    http_tunnels: HttpTunnelMap,
    http_clients: HttpClientMap,
}

impl Server {
    pub fn new(
        port_range: RangeInclusive<u16>,
        bind: IpAddr,
        backend_url: String,
        relay_key: Option<String>,
    ) -> Result<Self> {
        Ok(Self {
            port_range,
            bind,
            auth: Authentication::new(backend_url, relay_key)?,
            tcp_conns: Arc::new(DashMap::new()),
            http_tunnels: Arc::new(DashMap::new()),
            http_clients: Arc::new(DashMap::new()),
        })
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
            let (stream, addr) = match listener.accept().await {
                Ok(accepted) => accepted,
                Err(err) => {
                    // e.g. out of file descriptors; keep serving existing clients
                    warn!(%err, "failed to accept control connection");
                    sleep(Duration::from_millis(100)).await;
                    continue;
                }
            };
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
            Some(ClientMessage::Hello(msg)) => match msg.host_config {
                HostConfig::Tcp(tcp) => self.handle_tcp(stream, &msg.token, tcp).await,
                HostConfig::Http(http) => self.handle_http(stream, &msg.token, http).await,
            },
            Some(ClientMessage::Accept(id)) => self.handle_accept(stream, id).await,
            None => Ok(()),
        }
    }

    async fn handle_tcp(
        &self,
        mut stream: StreamWorker<TcpStream>,
        token: &str,
        tcp: TcpHostConfig,
    ) -> Result<()> {
        let mut pub_url = match self.auth.check_tcp(token, tcp.remote_port).await {
            Ok(url) => url,
            Err(err) => {
                warn!(%err, "authentication failed");
                stream
                    .send(RelayMessage::Error("authentication failed".to_string()))
                    .await?;
                return Ok(());
            }
        };

        let listener = match self.create_listener(tcp.remote_port).await {
            Ok(listener) => listener,
            Err(err) => {
                stream.send(RelayMessage::Error(err.into())).await?;
                return Ok(());
            }
        };
        let port = listener.local_addr()?.port();
        info!(?port, "new client");

        if tcp.remote_port.is_none() {
            pub_url = format!("{pub_url}:{port}");
        }

        stream.send(RelayMessage::Hello(pub_url)).await?;

        let mut heartbeat = interval(HEARTBEAT_INTERVAL);
        heartbeat.set_missed_tick_behavior(MissedTickBehavior::Delay);

        loop {
            tokio::select! {
                _ = heartbeat.tick() => {
                    if stream.send(RelayMessage::Heartbeat).await.is_err() {
                        return Ok(());
                    }
                }
                accepted = listener.accept() => {
                    let (stream2, addr) = match accepted {
                        Ok(accepted) => accepted,
                        Err(err) => {
                            warn!(%err, "failed to accept connection");
                            continue;
                        }
                    };
                    info!(?addr, ?port, "new connection");

                    let id = Uuid::new_v4();
                    self.tcp_conns.insert(id, stream2);
                    let conns = Arc::clone(&self.tcp_conns);
                    tokio::spawn(async move {
                        sleep(PENDING_CONNECTION_TTL).await;
                        if conns.remove(&id).is_some() {
                            warn!(%id, "removed stale connection");
                        }
                    });
                    if stream.send(RelayMessage::Connection(id)).await.is_err() {
                        return Ok(());
                    }
                }
                // The client never sends anything after the hello, so any
                // read result means it is gone (or misbehaving).
                _ = stream.recv::<ClientMessage>() => return Ok(()),
            }
        }
    }

    async fn handle_http(
        &self,
        mut stream: StreamWorker<TcpStream>,
        token: &str,
        http: HttpHostConfig,
    ) -> Result<()> {
        let domain = match self.auth.check_http(token, http.domain.clone()).await {
            Ok(domain) => domain.to_ascii_lowercase(),
            Err(err) => {
                warn!(%err, "authentication failed");
                stream
                    .send(RelayMessage::Error("authentication failed".to_string()))
                    .await?;
                return Ok(());
            }
        };

        let (tx, mut rx) = tokio::sync::mpsc::channel(32);
        let registration = Uuid::new_v4();
        match self.http_clients.entry(domain.clone()) {
            Entry::Occupied(_) => {
                warn!(?domain, "domain already in use");
                stream
                    .send(RelayMessage::Error("domain already in use".to_string()))
                    .await?;
                return Ok(());
            }
            Entry::Vacant(slot) => {
                slot.insert(HttpClient {
                    id: registration,
                    tx,
                    auth: http.auth,
                });
            }
        }
        info!(?domain, "new http client");

        let result = self.serve_http_client(&mut stream, &domain, &mut rx).await;

        // Only remove our own registration, never a newer one
        self.http_clients
            .remove_if(&domain, |_, client| client.id == registration);
        result
    }

    async fn serve_http_client(
        &self,
        stream: &mut StreamWorker<TcpStream>,
        domain: &str,
        rx: &mut tokio::sync::mpsc::Receiver<Uuid>,
    ) -> Result<()> {
        stream.send(RelayMessage::Hello(domain.to_string())).await?;

        let mut heartbeat = interval(HEARTBEAT_INTERVAL);
        heartbeat.set_missed_tick_behavior(MissedTickBehavior::Delay);

        loop {
            tokio::select! {
                _ = heartbeat.tick() => {
                    if stream.send(RelayMessage::Heartbeat).await.is_err() {
                        return Ok(());
                    }
                }
                id = rx.recv() => {
                    let Some(id) = id else { return Ok(()) };
                    if stream.send(RelayMessage::Connection(id)).await.is_err() {
                        return Ok(());
                    }
                }
                _ = stream.recv::<ClientMessage>() => return Ok(()),
            }
        }
    }

    async fn handle_accept(&self, stream: StreamWorker<TcpStream>, id: Uuid) -> Result<()> {
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
                io::copy_bidirectional(&mut parts.io, &mut stream2).await?;
            }
            None => warn!(%id, "missing connection"),
        }
        Ok(())
    }
}
