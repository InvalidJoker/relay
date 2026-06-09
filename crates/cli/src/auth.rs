use reqwest::Client;
use url::Url;

#[derive(serde::Deserialize)]
pub(crate) struct DeviceCodeResponse {
    pub(crate) device_code: String,
    pub(crate) user_code: String,
    //pub(crate) verification_uri: String,
    pub(crate) verification_uri_complete: String,
    pub(crate) interval: Option<u64>,
}

#[derive(serde::Deserialize)]
pub(crate) struct TokenResponse {
    pub(crate) access_token: Option<String>,
    pub(crate) error: Option<String>,
}

pub(crate) async fn request_device_code(
    server: Url,
    client: &Client,
) -> anyhow::Result<DeviceCodeResponse> {
    let response = client
        .post(server.join("/api/auth/device/code")?)
        .json(&serde_json::json!({
            "client_id": "cli",
            "scope": "openid profile"
        }))
        .send()
        .await?
        .json::<DeviceCodeResponse>()
        .await?;

    Ok(response)
}
