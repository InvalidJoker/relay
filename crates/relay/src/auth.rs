use relay_common::model::RelayType;
use reqwest::Client;
use std::time::Duration;

/// How long the panel gets to answer an authentication request.
const AUTH_TIMEOUT: Duration = Duration::from_secs(5);

pub struct Authentication {
    host: String,
    /// Identifies this relay to the panel so it can return the right domains.
    relay_key: Option<String>,
    client: Client,
}

#[derive(serde::Deserialize)]
pub(crate) struct CheckResponse {
    pub(crate) result: String,
}

impl Authentication {
    pub fn new(host: String, relay_key: Option<String>) -> anyhow::Result<Self> {
        Ok(Self {
            host,
            relay_key,
            client: Client::builder().timeout(AUTH_TIMEOUT).build()?,
        })
    }

    async fn check(
        &self,
        token: &str,
        provided: Option<String>,
        relay_type: RelayType,
    ) -> anyhow::Result<String> {
        let url = format!("{}/api/internal/auth/relay", self.host);
        let mut request = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {token}"))
            .header("X-Provided", provided.unwrap_or_default())
            .header("X-Relay-Type", relay_type.to_string());
        if let Some(key) = &self.relay_key {
            request = request.header("X-Relay-Key", key);
        }
        let resp = request.send().await?;

        if resp.status().is_success() {
            let check_response: CheckResponse = resp.json().await?;
            Ok(check_response.result)
        } else {
            Err(anyhow::anyhow!(
                "Authentication failed with response: {}",
                resp.text().await?
            ))
        }
    }

    pub async fn check_tcp(&self, token: &str, port: Option<u16>) -> anyhow::Result<String> {
        self.check(token, port.map(|p| p.to_string()), RelayType::Tcp)
            .await
    }

    pub async fn check_http(&self, token: &str, domain: Option<String>) -> anyhow::Result<String> {
        self.check(token, domain, RelayType::Http).await
    }
}
