use crate::config::base::config::Config;
use crate::errors::app_error::AppError;
use crate::middlewares::request_context::RequestContext;
use crate::models::echo::EchoRequestBody;
use axum::response::IntoResponse;
use axum::{Extension, Json, extract::rejection::JsonRejection};
use http::StatusCode;

pub async fn handle_echo(
    Extension(ctx): Extension<RequestContext>,
    payload: Result<Json<EchoRequestBody>, JsonRejection>,
) -> Result<impl IntoResponse, AppError> {
    let Json(body) = payload.map_err(|_| AppError {
        status: StatusCode::BAD_REQUEST,
        message: "Invalid JSON payload".to_string(),
    })?;
    let config = Config::from_env();
    tracing::info!(
        config.app_env,
        ctx.path,
        "headers {:?}",
        ctx.forward_headers
    );
    Ok(Json(body))
}
