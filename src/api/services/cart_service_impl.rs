use crate::api::services::cart_service::CartService;
use crate::common::http_client::base_http_client::BaseHttpClient;
use crate::common::http_client::http_client::HttpClient;
use crate::common::http_client::http_error::HttpError;
use crate::common::request_context::RequestContext;
use crate::domain::cart::{CartResponse, Product};
use crate::external_models::cart::{ExternalCartsResponse, ExternalProduct};
use async_trait::async_trait;
use http::HeaderMap;

pub struct CartServiceImpl {
    client: BaseHttpClient,
}

impl CartServiceImpl {
    pub fn new(client: BaseHttpClient) -> Self {
        CartServiceImpl { client }
    }

    fn default_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers
    }
    fn map_cart_response(
        &self,
        response: ExternalCartsResponse,
    ) -> Result<CartResponse, HttpError> {
        let first_cart = response
            .carts
            .get(0)
            .ok_or_else(|| HttpError::from_error("No carts found".into()))?;

        let products: Vec<ExternalProduct> = first_cart.products.clone();

        let cart_response = CartResponse {
            id: first_cart.id,
            products: self.map_items_to_cart_response(products)?,
        };
        Ok(cart_response)
    }

    fn map_items_to_cart_response(
        &self,
        items: Vec<ExternalProduct>,
    ) -> Result<Vec<Product>, HttpError> {
        let items: Vec<Product> = items
            .iter()
            .map(|item| {
                Ok(Product {
                    id: item.id.clone(),
                    quantity: item.quantity,
                })
            })
            .collect::<Result<Vec<Product>, _>>()?;

        Ok(items)
    }
}

#[async_trait]
impl CartService for CartServiceImpl {
    async fn get_cart(&self, ctx: &RequestContext) -> Result<CartResponse, HttpError> {
        let response = self
            .client
            .get("/cart", ctx, &self.default_headers())
            .await
            .map_err(|e| HttpError::from_reqwest_err(e))?;

        let cart_response: ExternalCartsResponse =
            serde_json::from_str(&response.body).map_err(|e| HttpError::from_error(e.into()))?;

        let cart: CartResponse = self.map_cart_response(cart_response.clone())?;

        Ok(cart)
    }
}
