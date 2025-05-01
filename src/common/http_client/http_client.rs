use crate::common::http_client::http_response::HttpResponse;
use crate::common::request_context::RequestContext;
use async_trait::async_trait;
use reqwest::header::HeaderMap;
use reqwest_middleware::Error;

#[async_trait]
pub trait HttpClient: Send + Sync {
    async fn get(
        &self,
        path: &str,
        ctx: &RequestContext,
        extra_headers: &HeaderMap,
    ) -> Result<HttpResponse, Error>;
    async fn post<B: serde::Serialize + Sync>(
        &self,
        path: &str,
        body: &B,
        ctx: &RequestContext,
        extra_headers: &HeaderMap,
    ) -> Result<HttpResponse, Error>;
    async fn put<B: serde::Serialize + Sync>(
        &self,
        path: &str,
        body: &B,
        ctx: &RequestContext,
        extra_headers: &HeaderMap,
    ) -> Result<HttpResponse, Error>;
    async fn patch<B: serde::Serialize + Sync>(
        &self,
        path: &str,
        body: &B,
        ctx: &RequestContext,
        extra_headers: &HeaderMap,
    ) -> Result<HttpResponse, Error>;
    async fn delete(
        &self,
        path: &str,
        ctx: &RequestContext,
        extra_headers: &HeaderMap,
    ) -> Result<HttpResponse, Error>;
}
