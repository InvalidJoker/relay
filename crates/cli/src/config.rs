use serde::{Deserialize, Serialize};
use relay_common::model::relay::RelayType;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    // serialize as type
    #[serde(rename = "type")]
    pub relay_type: RelayType,

    pub port: Option<u16>,

    pub subdomain: Option<String>,
    pub domain: Option<String>,
}
