use axum::Json;
use axum::response::IntoResponse;
use http::StatusCode;

pub async fn not_found_handler() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({ "error": "Not Found" })),
    )
}
