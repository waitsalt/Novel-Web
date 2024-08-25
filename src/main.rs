use core::panic;

use settings::SETTINGS;

mod database;
mod logger;
mod models;
mod routes;
mod settings;
mod web;

#[tokio::main]
async fn main() {
    let web = web::init().await;

    let port = SETTINGS.server.port;
    let listener = tokio::net::TcpListener::bind(format!("localhost:{port}"))
        .await
        .unwrap_or_else(|_| panic!("Failed to start listener"));

    println!("Server run in http://localhost:{port}");

    axum::serve(listener, web)
        .await
        .expect("Failed to start server\n");
}
