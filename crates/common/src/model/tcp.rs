use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TcpHostConfig {
    pub local_port: u16,

    /// Optional remote port (if None, we will randomly assign one)
    pub remote_port: Option<u16>,
}
