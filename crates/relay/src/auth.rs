use relay_common::model::RelayType;
use reqwest::Client;

pub struct Authentication {
    host: String,
    client: Client,
}

#[derive(serde::Deserialize)]
pub(crate) struct CheckResponse {
    pub(crate) result: String,
}

impl Authentication {
    pub fn new(host: String) -> Self {
        Self {
            host,
            client: Client::new(),
        }
    }

    async fn check(
        &self,
        token: &str,
        provided: Option<String>,
        relay_type: RelayType,
    ) -> anyhow::Result<String> {
        let url = format!("{}/api/internal/auth/relay", self.host);
        let resp = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .header("X-Provided", provided.unwrap_or_default())
            .header("X-Relay-Type", relay_type.to_string())
            .send()
            .await?;

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
