use crate::model::relay::HostConfig;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A message from the client on the relay
#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMessage {
    /// Response to an authentication challenge from the server.
    Authenticate(String),

    /// Initial client message specifying a port to forward.
    Hello(HostConfig),

    /// Accepts an incoming TCP connection, using this stream as a proxy.
    Accept(Uuid),
}

/// A message from the relay on the control connection.
#[derive(Debug, Serialize, Deserialize)]
pub enum RelayMessage {
    /// Response to a client's initial message, with actual public port.
    Hello(u16),

    /// No-op used to test if the client is still reachable.
    Heartbeat,

    /// Asks the client to accept a forwarded TCP connection.
    Connection(Uuid),

    /// Indicates a server error that terminates the connection.
    Error(String),
}
