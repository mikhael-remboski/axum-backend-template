use crate::api::services::cart_service::CartService;
use crate::common::errors::app_error::AppError;
use crate::common::request_context::RequestContext;
use crate::domain::cart::CartResponse;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use http::StatusCode;
use std::sync::Arc;

pub async fn handle_fetch_cart(
    cart_service: Extension<Arc<dyn CartService + Send + Sync>>,
    request_context: Extension<RequestContext>,
) -> Result<impl IntoResponse, AppError> {
    let response: CartResponse =
        cart_service
            .get_cart(&request_context)
            .await
            .map_err(|_| AppError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Failed to fetch cart".to_string(),
            })?;

    Ok(Json(response))
}
