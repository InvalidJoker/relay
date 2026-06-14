use relay_common::model::relay::RelayType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    // serialize as type
    #[serde(rename = "type")]
    pub relay_type: RelayType,

    pub port: Option<u16>,

    pub subdomain: Option<String>,
    pub domain: Option<String>,
}
