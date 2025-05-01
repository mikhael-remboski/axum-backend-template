use crate::errors::app_error::AppError;
use crate::models::echo::EchoRequestBody;
use crate::services::echo_service::EchoService;
use axum::response::IntoResponse;
use axum::{Extension, Json, extract::rejection::JsonRejection};
use http::StatusCode;
use std::sync::Arc;

pub async fn handle_echo(
    echo_service: Extension<Arc<dyn EchoService + Send + Sync>>,
    payload: Result<Json<EchoRequestBody>, JsonRejection>,
) -> Result<impl IntoResponse, AppError> {
    let Json(body) = payload.map_err(|_| AppError {
        status: StatusCode::BAD_REQUEST,
        message: "Invalid JSON payload".to_string(),
    })?;
    echo_service.echo(&body.message);
    Ok(Json(body))
}
