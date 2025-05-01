use crate::infra::handlers::echo_handler::handle_echo;
use axum::{Router, routing::post};

pub fn routes() -> Router {
    Router::new().route("/echo", post(handle_echo))
}
