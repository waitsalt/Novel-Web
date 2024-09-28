use axum::{routing::post, Router};

pub async fn router() -> Router {
    Router::new()
        .route("/user/create", post(create_user))
        .route("/user/login", post(login_user))
        .route("/user/info", post(info_user))
}

async fn create_user() {}

async fn login_user() {}

async fn info_user() {}
