use crate::logger;
use crate::routes;
use axum::Router;

pub async fn init() -> Router {
    logger::init();

    Router::new()
        .merge(routes::book::route())
        .merge(routes::user::route())
}
