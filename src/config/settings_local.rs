use crate::config::base::config::Config;
use crate::config::base::endpoint::{Endpoint, Endpoints};
use std::env;

pub fn load() -> Config {
    dotenvy::from_filename(".env").ok();

    Config {
        app_env: "local".to_string(),
        port: env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .expect("Port should be a number"),
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
