use crate::api::handlers::cart_handler::handle_fetch_cart;
use axum::Router;

pub fn routes() -> Router {
    Router::new().route("/cart", axum::routing::get(handle_fetch_cart))
}
