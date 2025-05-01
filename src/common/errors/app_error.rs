use crate::domain::error::ErrorResponse;
use axum::{
    Json,
    response::{IntoResponse, Response},
};
use http::StatusCode;

#[derive(Debug)]
pub struct AppError {
    pub status: StatusCode,
    pub message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let body = Json(ErrorResponse {
            error: self.message,
        });
        (self.status, body).into_response()
    }
}
