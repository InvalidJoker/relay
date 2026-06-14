use crate::model::HostConfig;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct HelloMessage {
    pub token: String,
    pub host_config: HostConfig,
}

/// A message from the client on the relay
#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMessage {
    /// Initial client message specifying a port to forward and authentication token.
    Hello(HelloMessage),

    /// Accepts an incoming TCP connection, using this stream as a proxy.
    Accept(Uuid),
}

/// A message from the relay on the control connection.
#[derive(Debug, Serialize, Deserialize)]
pub enum RelayMessage {
    /// Response to a client's initial message, with actual public port or domain.
    Hello(String),

    /// No-op used to test if the client is still reachable.
    Heartbeat,

    /// Asks the client to accept a forwarded TCP connection.
    Connection(Uuid),

    /// Indicates a server error that terminates the connection.
    Error(String),
}
