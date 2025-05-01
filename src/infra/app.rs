use crate::config::base::config::Config;
use crate::infra::middlewares::cors::cors_layer;
use crate::common::request_context::request_context_middleware;

use crate::api::ports::echo_usecase_port::EchoPort;
use crate::api::usecases::echo_usecase::EchoUseCase;
use crate::infra::handlers::not_found_handler::not_found_handler;
use crate::infra::routes::{echo, health};
use axum::{Extension, Router};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

pub fn create_app() -> Router {
    let config = Config::from_env();

    let echo_service: Arc<dyn EchoPort + Send + Sync> = Arc::new(EchoUseCase::new(config));

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
