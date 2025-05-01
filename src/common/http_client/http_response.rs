use http::StatusCode;
use reqwest::header::HeaderMap;

pub struct HttpResponse {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: String,
}
