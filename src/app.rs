use crate::config::base::config::Config;
use crate::handlers::not_found_handler::not_found_handler;
use crate::middlewares::cors::cors_layer;
use crate::middlewares::request_context::request_context_middleware;
use crate::routes::{echo, health};
use crate::services::echo_service::EchoService;
use crate::services::echo_service_impl::EchoServiceImpl;
use axum::{Extension, Router};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

pub fn create_app() -> Router {
    let config = Config::from_env();

    let echo_service: Arc<dyn EchoService + Send + Sync> = Arc::new(EchoServiceImpl::new(config));

    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(cors_layer())
        .layer(Extension(echo_service.clone()))
        .layer(axum::middleware::from_fn(request_context_middleware));

    Router::new()
        .merge(health::routes())
        .merge(echo::routes())
        .layer(middleware_stack)
        .fallback(not_found_handler)
}
