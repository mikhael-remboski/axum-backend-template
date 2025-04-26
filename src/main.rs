mod app;
mod errors;
mod handlers;
mod middlewares;
mod models;
mod routes;
mod utils;

use app::create_app;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    utils::logger::init();

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let port: u16 = port.parse().expect("PORT debe ser un número");
    let addr = format!("0.0.0.0:{}", port);

    let app = create_app();

    let listener = TcpListener::bind(addr).await.unwrap();
    tracing::info!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
