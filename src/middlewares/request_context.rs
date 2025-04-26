use crate::config::base::config::Config;
use axum::body::Body;
use axum::middleware::Next;
use axum::response::Response;
use http::Request;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct RequestContext {
    pub trace_id: String,
    pub forward_headers: HashMap<String, String>,
    pub path: String,
}

pub async fn request_context_middleware(mut req: Request<Body>, next: Next) -> Response<Body> {
    let uuid_v4 = Uuid::new_v4().to_string();

    let trace_id = req
        .headers()
        .get("x-trace-id")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or(uuid_v4);

    let forward_headers = Config::from_env().forward_headers;

    let mut forward_headers_values = HashMap::new();

    for header_name in forward_headers {
        if let Some(header_value) = req.headers().get(&header_name) {
            forward_headers_values.insert(
                header_name,
                header_value.to_str().unwrap_or_default().to_string(),
            );
        }
    }

    let ctx = RequestContext {
        trace_id,
        path: req.uri().path().to_string(),
        forward_headers: forward_headers_values,
    };

    req.extensions_mut().insert(ctx);

    next.run(req).await
}
