mod app;
mod config;
mod errors;
mod handlers;
mod middlewares;
mod models;
mod routes;
mod services;
mod utils;
use crate::config::base::config::Config;
use app::create_app;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    utils::logger::init();

    let config = Config::from_env();

    let addr = format!("0.0.0.0:{}", config.port);

    let app = create_app();

    let listener = TcpListener::bind(addr).await.unwrap();
    tracing::info!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
