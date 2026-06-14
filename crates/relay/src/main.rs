use crate::server::Server;
use std::env;
use std::net::IpAddr;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod auth;
mod http_proxy;
mod server;

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
