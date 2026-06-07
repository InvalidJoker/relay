use crate::Backend;
use axum::Router;

pub(crate) fn config() -> Router<Backend> {
    Router::new().nest("/api", Router::new())
}
