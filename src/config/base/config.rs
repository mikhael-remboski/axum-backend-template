use crate::config::base::endpoint::Endpoints;
#[derive(Debug, Clone)]
pub struct Config {
    pub app_env: String,
    pub port: u16,
    pub endpoints: Endpoints,
    pub forward_headers: Vec<String>,
}
