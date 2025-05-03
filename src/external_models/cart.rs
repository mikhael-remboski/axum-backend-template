use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExternalProduct {
    pub id: u32,
    pub quantity: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExternalCartResponse {
    pub id: u32,
    pub products: Vec<ExternalProduct>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExternalCartsResponse {
    pub carts: Vec<ExternalCartResponse>,
}
