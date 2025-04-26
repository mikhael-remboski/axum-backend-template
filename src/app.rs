use crate::handlers::not_found_handler::not_found_handler;
use crate::middlewares::cors::cors_layer;
use crate::routes::{echo, health};
use axum::Router;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

pub fn create_app() -> Router {
    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(cors_layer());

    Router::new()
        .merge(health::routes())
        .merge(echo::routes())
        .layer(middleware_stack)
        .fallback(not_found_handler)
}
