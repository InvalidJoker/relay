use std::net::IpAddr;
use std::ops::RangeInclusive;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tracing::{info, info_span, warn, Instrument};
use relay_common::connection::ClientMessage;
use relay_common::constants::RELAY_PORT;
use anyhow::Result;
use relay_common::worker::StreamWorker;

pub struct Server {
    /// The port range to listen on.
    port_range: RangeInclusive<u16>,

    /// The address to bind everything to
    bind: IpAddr,
}

impl Server {
    pub fn new(port_range: RangeInclusive<u16>, bind: IpAddr) -> Self {
        Self {
            port_range,
            bind,
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
            None => Ok(()),
            Some(ClientMessage::Authenticate(_)) => {
                warn!("unexpected authenticate");
                Ok(())
            }
            Some(_) => {
                warn!("unexpected message");
                Ok(())
            }
        }
    }
}