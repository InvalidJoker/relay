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

    /// Concurrent map of IDs to incoming connections.
    conns: Arc<DashMap<Uuid, TcpStream>>,
}

impl Server {
    pub fn new(port_range: RangeInclusive<u16>, bind: IpAddr, backend_url: String) -> Self {
        Self {
            port_range,
            bind,
            auth: Authentication::new(backend_url),
            conns: Arc::new(DashMap::new()),
        }
    }

    pub async fn listen(self) -> Result<()> {
        let this = Arc::new(self);
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
            // Client requests any available port in range.
            //
            // In this case, we bind to 150 random port numbers. We choose this value because in
            // order to find a free port with probability at least 1-δ, when ε proportion of the
            // ports are currently available, it suffices to check approximately -2 ln(δ) / ε
            // independently and uniformly chosen ports (up to a second-order term in ε).
            //
            // Checking 150 times gives us 99.999% success at utilizing 85% of ports under these
            // conditions, when ε=0.15 and δ=0.00001.
            for _ in 0..150 {
                let port = fastrand::u16(self.port_range.clone());
                match try_bind(port).await {
                    Ok(listener) => return Ok(listener),
                    Err(_) => continue,
                }
            }
            Err("failed to find an available port")
        }
    }

    async fn handle_connection(&self, stream: TcpStream) -> Result<()> {
        let mut stream = StreamWorker::new(stream);
        /*if let Some(auth) = &self.auth {
            if let Err(err) = auth.server_handshake(&mut stream).await {
                warn!(%err, "server handshake failed");
                stream.send(ServerMessage::Error(err.to_string())).await?;
                return Ok(());
            }
        }*/

        match stream.recv_timeout().await? {
            Some(ClientMessage::Hello(msg)) => {
                match msg.host_config {
                    HostConfig::Tcp(tcp) => {
                        if let Err(err) = self.auth.check_tcp(&msg.token, tcp.remote_port).await {
                            warn!(%err, "authentication failed");
                            stream
                                .send(relay_common::connection::RelayMessage::Error(
                                    err.to_string(),
                                ))
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
                        stream.send(RelayMessage::Hello(port)).await?;

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
                                let conns = Arc::clone(&self.conns);

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
                    HostConfig::Http(_) => {
                        warn!("http not supported yet");
                        stream
                            .send(RelayMessage::Error("http not supported yet".to_string()))
                            .await?;
                        Ok(())
                    }
                }
            }
            Some(ClientMessage::Accept(id)) => {
                info!(%id, "forwarding connection");
                match self.conns.remove(&id) {
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
