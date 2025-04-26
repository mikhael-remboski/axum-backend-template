use crate::config::base::config::Config;
use crate::config::{settings_local, settings_prod, settings_stg};
use std::env;

impl Config {
    pub fn from_env() -> Self {
        let app_env = env::var("APP_ENV").unwrap_or_else(|_| "local".to_string());

        match app_env.as_str() {
            "local" => settings_local::load(),
            "stg" => settings_stg::load(),
            "prod" => settings_prod::load(),
            _ => {
                panic!("APP_ENV invalid, expected 'local', 'stg' or 'prod'");
            }
        }
    }
}
