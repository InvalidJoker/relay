use relay_common::model::RelayType;
use serde::{Deserialize, Serialize};
use relay_common::model::http::AuthConfig;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    // serialize as type
    #[serde(rename = "type")]
    pub relay_type: RelayType,

    pub port: u16,

    pub remote_port: Option<u16>,

    pub auth: Option<AuthConfig>,
    pub domain: Option<String>,
}
