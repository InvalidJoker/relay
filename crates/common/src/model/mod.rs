pub mod http;
pub mod tcp;

use crate::model::http::HttpHostConfig;
use crate::model::tcp::TcpHostConfig;
use core::fmt;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelayType {
    Http,
    Tcp,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum HostConfig {
    Tcp(TcpHostConfig),
    Http(HttpHostConfig),
}

impl fmt::Display for RelayType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RelayType::Http => write!(f, "http"),
            RelayType::Tcp => write!(f, "tcp"),
        }
    }
}
