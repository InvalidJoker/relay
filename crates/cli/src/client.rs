use anyhow::bail;
use anyhow::{Context, Result};
use relay_common::connection::{ClientMessage, RelayMessage};
use relay_common::constants::{NETWORK_TIMEOUT, RELAY_PORT};
use relay_common::model::relay::{HelloMessage, HostConfig};
use relay_common::worker::StreamWorker;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::time::timeout;
use tracing::{Instrument, error, info, info_span, warn};
use uuid::Uuid;

pub struct Client {
    /// Control connection to the server.
    conn: Option<StreamWorker<TcpStream>>,

    /// Destination address of the server.
    to: String,

    // Local host that is forwarded.
    local_host: String,

    /// Local port that is forwarded.
    local_port: u16,
}

impl Client {
    /// Create a new client.
    pub async fn new(
        local_host: &str,
        local_port: u16,
        to: &str,
        host_config: HostConfig,
        secret: String,
    ) -> Result<Self> {
        let mut stream = StreamWorker::new(connect_with_timeout(to, RELAY_PORT).await?);

        let msg = HelloMessage {
            token: secret,
            host_config,
        };

        stream.send(ClientMessage::Hello(msg)).await?;
        let remote_target = match stream.recv_timeout().await? {
            Some(RelayMessage::Hello(remote_target)) => remote_target,
            Some(RelayMessage::Error(message)) => bail!("server error: {message}"),
            Some(_) => bail!("unexpected initial non-hello message"),
            None => bail!("unexpected EOF"),
        };
        info!("connected to server");
        info!("listening at {remote_target} -> {local_host}:{local_port}");

        Ok(Client {
            conn: Some(stream),
            to: to.to_string(),
            local_host: local_host.to_string(),
            local_port,
        })
    }

    /// Start the client, listening for new connections.
    pub async fn listen(mut self) -> Result<()> {
        let mut conn = self.conn.take().unwrap();
        let this = Arc::new(self);
        loop {
            match conn.recv().await? {
                Some(RelayMessage::Hello(_)) => warn!("unexpected hello"),
                Some(RelayMessage::Heartbeat) => (),
                Some(RelayMessage::Connection(id)) => {
                    let this = Arc::clone(&this);
                    tokio::spawn(
                        async move {
                            info!("new connection");
                            match this.handle_connection(id).await {
                                Ok(_) => info!("connection exited"),
                                Err(err) => warn!(%err, "connection exited with error"),
                            }
                        }
                        .instrument(info_span!("proxy", %id)),
                    );
                }
                Some(RelayMessage::Error(err)) => error!(%err, "server error"),
                None => return Ok(()),
            }
        }
    }

    async fn handle_connection(&self, id: Uuid) -> Result<()> {
        let mut remote_conn =
            StreamWorker::new(connect_with_timeout(&self.to[..], RELAY_PORT).await?);
        remote_conn.send(ClientMessage::Accept(id)).await?;
        let mut local_conn = connect_with_timeout(&self.local_host, self.local_port).await?;
        let mut parts = remote_conn.into_parts();
        debug_assert!(parts.write_buf.is_empty(), "framed write buffer not empty");
        local_conn.write_all(&parts.read_buf).await?; // mostly of the cases, this will be empty
        tokio::io::copy_bidirectional(&mut local_conn, &mut parts.io).await?;
        Ok(())
    }
}

async fn connect_with_timeout(to: &str, port: u16) -> Result<TcpStream> {
    match timeout(NETWORK_TIMEOUT, TcpStream::connect((to, port))).await {
        Ok(res) => res,
        Err(err) => Err(err.into()),
    }
    .with_context(|| format!("could not connect to {to}:{port}"))
}
