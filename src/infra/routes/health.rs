use crate::infra::handlers::health_handler::health_check;
use axum::{Router, routing::get};

pub fn routes() -> Router {
    Router::new().route("/health", get(health_check))
}
