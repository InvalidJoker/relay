use core::fmt;
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpAuthConfig {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpHostConfig {
    pub local_port: u16,

    /// Optional custom domain (subdomain will be assigned if None)
    pub domain: Option<String>,

    /// Optional authentication config (if None, no authentication will be required)
    pub auth: Option<HttpAuthConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum HostConfig {
    Tcp(TcpHostConfig),
    Http(HttpHostConfig),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HelloMessage {
    pub token: String,
    pub host_config: HostConfig,
}

impl fmt::Display for RelayType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RelayType::Http => write!(f, "http"),
            RelayType::Tcp => write!(f, "tcp"),
        }
    }
}
