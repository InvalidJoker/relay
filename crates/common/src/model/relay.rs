use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelayType {
    Http,
    Tcp,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HostConfig {
    pub relay_type: RelayType,
    pub local_port: u16,

    /// If None, the server will assign a random port.
    pub remote_port: Option<u16>,

    /// Optional domain for HTTP relay (e.g., "example.com")
    pub domain: Option<String>,
}