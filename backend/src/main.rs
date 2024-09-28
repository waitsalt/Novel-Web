mod app;
mod database;
mod error;
mod logger;
mod model;
mod router;
mod setting;
mod util;

use setting::SETTING;

#[tokio::main]
async fn main() {
    let app = app::init().await;
    let addr = format!("127.0.0.1:{}", SETTING.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    tracing::debug!("server runing http://{addr}");
    axum::serve(listener, app).await.unwrap();
}
