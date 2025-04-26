use crate::errors::app_error::AppError;
use crate::models::echo::EchoRequestBody;
use axum::response::IntoResponse;
use axum::{Json, extract::rejection::JsonRejection};
use http::StatusCode;
use tracing::info;

pub async fn handle_echo(
    payload: Result<Json<EchoRequestBody>, JsonRejection>,
) -> Result<impl IntoResponse, AppError> {
    let Json(body) = payload.map_err(|_| AppError {
        status: StatusCode::BAD_REQUEST,
        message: "Invalid JSON payload".to_string(),
    })?;
    info!("request received {:?}", body);
    Ok(Json(body))
}
