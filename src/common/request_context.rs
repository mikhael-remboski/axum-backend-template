use crate::config::base::config::Config;
use axum::{body::Body, middleware::Next, response::Response};
use http::Request;
use std::collections::HashMap;
use tracing::{Instrument, Span, info_span};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct RequestContext {
    pub forward_headers: HashMap<String, String>,
}

pub async fn request_context_middleware(mut req: Request<Body>, next: Next) -> Response<Body> {
    let config = Config::from_env();

    let trace_id = extract_or_generate_trace_id(&req);
    let forward_headers = extract_forward_headers(&req, &config.forward_headers);

    let ctx = RequestContext { forward_headers };

    req.extensions_mut().insert(ctx);

    let span = build_request_span(&trace_id, req.uri().path(), &config.app_env);

    next.run(req).instrument(span).await
}

fn extract_or_generate_trace_id<B>(req: &Request<B>) -> String {
    let uuid_v4 = Uuid::new_v4().to_string();

    req.headers()
        .get("x-trace-id")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or(uuid_v4)
}

fn extract_forward_headers<B>(
    req: &Request<B>,
    headers_to_forward: &[String],
) -> HashMap<String, String> {
    let mut forwarded = HashMap::new();

    for header_name in headers_to_forward {
        if let Some(header_value) = req.headers().get(header_name) {
            forwarded.insert(
                header_name.clone(),
                header_value.to_str().unwrap_or_default().to_string(),
            );
        }
    }

    forwarded
}

fn build_request_span(trace_id: &str, path: &str, env: &str) -> Span {
    info_span!(
        "request",
        trace_id = %trace_id,
        path = %path,
        env = %env,
    )
}
