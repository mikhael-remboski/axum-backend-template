use crate::config::base::config::Config;
use crate::config::base::endpoint::{Endpoint, Endpoints};
use std::env;

pub fn load() -> Config {
    Config {
        app_env: "prod".to_string(),
        port: std::env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .expect("Invalid PORT"),
        endpoints: Endpoints {
            cart: Endpoint {
                host: env::var("CART_SERVICE_HOST").expect("CART_SERVICE_HOST required"),
                timeout: 5000,
                retries: 3,
                retry_delay: 1000,
            },
        },
    }
}
