use crate::common::http_client::http_client::HttpClient;
use crate::common::http_client::http_response::HttpResponse;
use crate::common::request_context::RequestContext;
use async_trait::async_trait;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};
use serde::Serialize;
use std::time::Duration;

#[derive(Clone)]
pub struct BaseHttpClient {
    client: ClientWithMiddleware,
    base_url: String,
}

impl BaseHttpClient {
    pub fn new(
        base_url: String,
        timeout_secs: u64,
        retries: u32,
        min_retry_delay: u64,
        max_retry_delay: u64,
    ) -> Self {
        let retry_policy = ExponentialBackoff::builder()
            .retry_bounds(
                Duration::from_millis(min_retry_delay),
                Duration::from_secs(max_retry_delay),
            )
            .build_with_max_retries(retries);

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .build()
            .expect("Failed to build HTTP client");

        let middleware_client = ClientBuilder::new(client)
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
            .build();

        Self {
            client: middleware_client,
            base_url,
        }
    }

    fn build_url(&self, path: &str) -> String {
        format!(
            "{}/{}",
            self.base_url.trim_end_matches('/'),
            path.trim_start_matches('/')
        )
    }

    fn build_headers(&self, ctx: &RequestContext, extra: &HeaderMap) -> HeaderMap {
        let mut header_map = HeaderMap::new();

        for (key, value) in ctx.forward_headers.iter() {
            if let (Ok(name), Ok(val)) = (
                HeaderName::from_bytes(key.as_bytes()),
                HeaderValue::from_str(value),
            ) {
                header_map.insert(name, val);
            }
        }

        for (name, val) in extra.iter() {
            header_map.insert(name.clone(), val.clone());
        }

        header_map
    }

    async fn send_request(
        &self,
        req: reqwest_middleware::RequestBuilder,
    ) -> Result<HttpResponse, reqwest_middleware::Error> {
        let resp = req.send().await?.error_for_status()?;
        let status = resp.status();
        let headers = resp.headers().clone();
        let body = resp.text().await?;
        Ok(HttpResponse {
            status,
            headers,
            body,
        })
    }
}

#[async_trait]
impl HttpClient for BaseHttpClient {
    async fn get(
        &self,
        path: &str,
        ctx: &RequestContext,
        extra_headers: &HeaderMap,
    ) -> Result<HttpResponse, reqwest_middleware::Error> {
        let url = self.build_url(path);
        let headers = self.build_headers(ctx, extra_headers);
        let req = self.client.get(url).headers(headers);
        self.send_request(req).await
    }

    async fn post<B: Serialize + Sync>(
        &self,
        path: &str,
        body: &B,
        ctx: &RequestContext,
        extra_headers: &HeaderMap,
    ) -> Result<HttpResponse, reqwest_middleware::Error> {
        let url = self.build_url(path);
        let headers = self.build_headers(ctx, extra_headers);
        let req = self.client.post(url).headers(headers).json(body);
        self.send_request(req).await
    }

    async fn put<B: Serialize + Sync>(
        &self,
        path: &str,
        body: &B,
        ctx: &RequestContext,
        extra_headers: &HeaderMap,
    ) -> Result<HttpResponse, reqwest_middleware::Error> {
        let url = self.build_url(path);
        let headers = self.build_headers(ctx, extra_headers);
        let req = self.client.put(url).headers(headers).json(body);
        self.send_request(req).await
    }

    async fn patch<B: Serialize + Sync>(
        &self,
        path: &str,
        body: &B,
        ctx: &RequestContext,
        extra_headers: &HeaderMap,
    ) -> Result<HttpResponse, reqwest_middleware::Error> {
        let url = self.build_url(path);
        let headers = self.build_headers(ctx, extra_headers);
        let req = self.client.patch(url).headers(headers).json(body);
        self.send_request(req).await
    }

    async fn delete(
        &self,
        path: &str,
        ctx: &RequestContext,
        extra_headers: &HeaderMap,
    ) -> Result<HttpResponse, reqwest_middleware::Error> {
        let url = self.build_url(path);
        let headers = self.build_headers(ctx, extra_headers);
        let req = self.client.delete(url).headers(headers);
        self.send_request(req).await
    }
}
