use axum::{routing::post, Json, Router};

use crate::error::{self, Error};
use crate::model::user::{ClaimsUser, CreateUser, PublicUser, VerifyUser};
use crate::setting::SETTING;
use crate::util::auth::create;

pub async fn router() -> Router {
    Router::new()
        .route("/user/signup", post(signup_user))
        .route("/user/signin", post(signin_user))
        .route("/user/info", post(info_user))
}

async fn signup_user(Json(create_user): Json<CreateUser>) -> Result<String, Error> {
    todo!()
}

async fn signin_user(Json(verify_user): Json<VerifyUser>) -> Result<String, Error> {
    let Json(user) = verify_user
        .query_user()
        .await
        .map_err(|_| Error::LoginError(error::LoginError::UserError))?;
    if user.password != verify_user.password {
        return Err(Error::LoginError(error::LoginError::PasswordError));
    }
    let public_user = PublicUser::new(user);
    let token = create(public_user, SETTING.auth.secret.as_str())
        .map_err(|_| error::AuthError::AuthCreation)?;
    Ok(token)
}

async fn info_user(claims_user: Option<ClaimsUser>) -> Result<Json<ClaimsUser>, Error> {
    match claims_user {
        Some(a) => Ok(Json(a)),
        None => Err(Error::AuthError(error::AuthError::AuthMiss)),
    }
}
