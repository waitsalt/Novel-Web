use crate::logger;
use axum::routing::{get, Router};
use tower_http::trace;

pub async fn init() -> Router {
    logger::init();

    Router::new()
        .merge(Router::new().route("/", get(root)))
        .layer(
            trace::TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().include_headers(true))
                .on_request(trace::DefaultOnRequest::new().level(tracing::Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO)),
        )
}

async fn root() -> &'static str {
    "nice to meet you!"
}
