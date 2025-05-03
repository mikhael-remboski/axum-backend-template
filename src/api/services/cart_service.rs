use crate::common::http_client::http_error::HttpError;
use crate::common::request_context::RequestContext;
use crate::domain::cart::CartResponse;
use async_trait::async_trait;

#[async_trait]
pub trait CartService: Send + Sync {
    async fn get_cart(&self, ctx: &RequestContext) -> Result<CartResponse, HttpError>;
}
