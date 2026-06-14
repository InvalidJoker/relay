use crate::server::Server;
use std::env;
use std::net::IpAddr;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod auth;
mod http_proxy;
mod server;
/*
IDEA:
- relay is the tcp proxy which runs under relay.invalidjoker.dev for example
- it will forward the traffic to the actual server, which is running on localhost:8080 for example

- for authetication (start hosting authentication) we request to the backend
- we initiallaly will only support 1 relay connected


 Local Idea:
 add a relay.toml which stores everything so we can only do relay run --config and the cli will autonatilcy get everything and we can run relay

Feature Ideas:
- for http we add support for basic auth
- persistent url


relay.invalidjoker.dev:anyport -> this
*.relay.invalidjoker.dev:80/443 -> this (only for http, we can do basic auth here and then forward to the actual server)
 */

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::filter::LevelFilter::from_level(
            if cfg!(debug_assertions) || env::var("DEBUG").unwrap_or_default().eq("1") {
                tracing::Level::DEBUG
            } else {
                tracing::Level::INFO
            },
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Unable to install default crypto provider");

    info!("Starting Relay on version {}", env!("GIT_HASH"));

    let port_range = 10000..=20000;
    let bind = IpAddr::from([0, 0, 0, 0]);
    let backend_url =
        env::var("BACKEND_URL").unwrap_or_else(|_| "http://localhost:5173".to_string());

    let server = Server::new(port_range, bind, backend_url);
    server.listen().await?;

    Ok(())
}
