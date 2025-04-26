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
    utils::logger::init();

    let app = create_app();

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    tracing::info!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
