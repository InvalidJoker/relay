use crate::{Backend, routes};
use axum::{Router, extract::Request};
use axum_server::{Handle, Server};
use core::time::Duration;
use std::net::SocketAddr;
use anyhow::Context;
use http::Method;
use tokio::sync::broadcast::error::RecvError;
use tower_http::{
    cors::{AllowHeaders, AllowOrigin, CorsLayer},
    trace::TraceLayer,
};
use tracing::{error, info};

impl Backend {
    pub fn router(&self) -> Router {
        let cors = CorsLayer::new()
            .allow_origin(AllowOrigin::any())
            .allow_methods([
                Method::GET,
                Method::POST,
                Method::PUT,
                Method::DELETE,
                Method::PATCH,
            ])
            //.allow_credentials(AllowCredentials::yes())
            .allow_headers(AllowHeaders::any());

        let router = Router::new()
            .merge(routes::config())
            .layer(TraceLayer::new_for_http())
            .layer(cors);

        router.with_state(self.clone())
    }

    pub async fn serve_http(&self) -> Result<(), anyhow::Error> {

        let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

        let listener = tokio::net::TcpListener::bind(addr).await?;

        info!("Listening on {}", addr);

        axum::serve(listener, self.router())
            .with_graceful_shutdown(async {
                tokio::signal::ctrl_c()
                    .await
                    .expect("Failed to listen for shutdown signal");
            })
            .await
            .context("HTTP server error")
    }
}