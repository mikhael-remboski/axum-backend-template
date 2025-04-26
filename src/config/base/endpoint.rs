#[derive(Debug, Clone)]
pub struct Endpoint {
    pub host: String,
    pub timeout: u64,
    pub retries: u16,
    pub retry_delay: u64,
}

#[derive(Debug, Clone)]
pub struct Endpoints {
    pub cart: Endpoint,
}
