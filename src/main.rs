mod api;
mod common;
mod config;
mod domain;
mod infra;

use crate::config::base::config::Config;
use infra::app::create_app;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    common::logger::init();

    let config = Config::from_env();

    let addr = format!("0.0.0.0:{}", config.port);

    let app = create_app();

    let listener = TcpListener::bind(addr).await.unwrap();
    tracing::info!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
