use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelayType {
    Http,
    Tcp,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TcpHostConfig {
    pub local_port: u16,

    /// Optional remote port (if None, we will randomly assign one)
    pub remote_port: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpHostConfig {
    pub local_port: u16,

    /// Optional custom domain (subdomain will be assigned if None)
    pub domain: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "config")]
pub enum HostConfig {
    Tcp(TcpHostConfig),
    Http(HttpHostConfig),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HelloMessage {
    pub token: String,
    pub host_config: HostConfig,
}