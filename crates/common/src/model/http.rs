use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HostConfig {
    pub local_port: u16,

    /// Optional custom domain (subdomain will be assigned if None)
    pub domain: Option<String>,

    /// Optional authentication config (if None, no authentication will be required)
    pub auth: Option<AuthConfig>,
}
