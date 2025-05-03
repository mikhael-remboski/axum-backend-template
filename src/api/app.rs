use crate::api::middlewares::cors::cors_layer;
use crate::common::request_context::request_context_middleware;
use crate::config::base::config::Config;

use crate::api::handlers::not_found_handler::not_found_handler;
use crate::api::routes::{cart, echo, health};
use crate::api::services::cart_service::CartService;
use crate::api::services::cart_service_impl::CartServiceImpl;
use crate::api::services::echo_service::EchoService;
use crate::api::services::echo_service_impl::EchoServiceImpl;
use crate::common::http_client::base_http_client::BaseHttpClient;
use axum::{Extension, Router};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

pub fn create_app() -> Router {
    let config = Config::from_env();

    let cart_client = BaseHttpClient::new(
        config.endpoints.cart.host.clone(),
        config.endpoints.cart.timeout,
        config.endpoints.cart.retries,
        config.endpoints.cart.min_retry_delay,
        config.endpoints.cart.max_retry_delay,
    );

    let cart_service: Arc<dyn CartService + Send + Sync> = Arc::new(CartServiceImpl::new(cart_client));

    let echo_service: Arc<dyn EchoService + Send + Sync> = Arc::new(EchoServiceImpl::new(config));

    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(cors_layer())
        .layer(Extension(echo_service))
        .layer(Extension(cart_service))
        .layer(axum::middleware::from_fn(request_context_middleware));

    Router::new()
        .merge(health::routes())
        .merge(echo::routes())
        .merge(cart::routes())
        .layer(middleware_stack)
        .fallback(not_found_handler)
}
