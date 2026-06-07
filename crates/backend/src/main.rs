use anyhow::Context;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::sync::Arc;
use tracing::{info, warn};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod error;
mod http;
mod routes;

#[derive(Clone)]
pub(crate) struct Backend {
    pub(crate) db: Arc<PgPool>,
}

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

    info!("Starting Backend on version {}", env!("GIT_HASH"));

    let database_url =
        std::env::var("DATABASE_URL").context("DATABASE_URL environment variable is not set")?;

    let database = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .context("Unable to connect to database")?;

    let backend = Backend {
        db: Arc::new(database),
    };

    /*sqlx::migrate!("./migrations")
    .run(&*database)
    .await
    .expect("Unable to run migrations");*/

    info!("Starting Webserver...");

    backend
        .serve_http()
        .await
        .expect("Unable to serve http server");

    info!("Backend shutdown complete");

    Ok(())
}
