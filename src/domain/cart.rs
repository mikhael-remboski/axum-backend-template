use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Product {
    pub id: u32,
    pub quantity: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CartResponse {
    pub id: u32,
    pub products: Vec<Product>,
}
