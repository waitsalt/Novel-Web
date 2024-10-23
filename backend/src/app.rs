use crate::database;
use crate::logger;
use crate::router;

use axum::routing::{get, Router};
use tower_http::trace;

pub async fn init() -> Router {
    logger::init().await;
    database::init().await;

    let api_router = Router::new()
        .merge(Router::new().route("/", get(root)))
        .merge(router::user::router().await)
        .merge(router::book::router().await)
        .layer(
            trace::TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().include_headers(true))
                .on_request(trace::DefaultOnRequest::new().level(tracing::Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO)),
        );
    Router::new().nest("/api", api_router)
}

async fn root() -> &'static str {
    "nice to meet you!"
}
