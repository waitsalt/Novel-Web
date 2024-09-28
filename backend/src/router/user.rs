use axum::{routing::post, Json, Router};

use crate::error::Error;
use crate::model::user::{ClaimsUser, User, VerifyUser};
use crate::setting::SETTING;
use crate::util::auth::create;

pub async fn router() -> Router {
    Router::new()
        .route("/user/create", post(create_user))
        .route("/user/login", post(login_user))
        .route("/user/info", post(info_user))
        .route("/user/test", post(test))
}

async fn create_user() {}

async fn login_user(Json(verify_user): Json<VerifyUser>) -> Result<String, Error> {
    if verify_user.name == "a" && verify_user.password == "1" {
        let user = User {
            id: "1".to_string(),
            name: "a".to_string(),
            level: 1,
            email: "1@1.com".to_string(),
            password: "1".to_string(),
        };
        let token = create(user, SETTING.auth.secret.as_str()).unwrap();
        return Ok(token);
    }
    tracing::info!("{:?}", verify_user);
    Err(Error::AuthError(crate::error::AuthError::AuthMiss))
}

async fn info_user() {}

async fn test(claims_user: Option<ClaimsUser>) {
    if claims_user.is_none() {
        tracing::info!("error");
        return;
    }
    tracing::info!("good");
}
