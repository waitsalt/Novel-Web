use axum::{routing::post, Json, Router};

use crate::error::{self, Error};
use crate::model::user::{ClaimsUser, CreateUser, PublicUser, VerifyUser};
use crate::setting::SETTING;
use crate::util::auth::{check_claims_user, create};

pub async fn router() -> Router {
    Router::new()
        .route("/user/signup", post(signup))
        .route("/user/signin", post(signin))
        .route("/user/info", post(info))
}

// test:
// curl -X POST http://127.0.0.1:8080/user/signup -H "Content-Type: application/json"
async fn signup(Json(create_user): Json<CreateUser>) -> Result<&'static str, Error> {
    let users = create_user
        .query_users()
        .await
        .map_err(|_| error::Error::SignupError(error::SignupError::QueryError))?;

    let num = users.len();
    if num == 1 {
        return Err(error::Error::SignupError(error::SignupError::UserExist));
    }

    let _ = create_user
        .create_user()
        .await
        .map_err(|_| error::SignupError::EmailExist)?;

    Ok("success")
}

// test:
// curl -X POST http://127.0.0.1:8080/user/signin -H "Content-Type: application/json"
async fn signin(Json(verify_user): Json<VerifyUser>) -> Result<String, Error> {
    let Json(user) = verify_user
        .query_user()
        .await
        .map_err(|_| error::SigninError::UserError)?;
    if user.password != verify_user.password {
        return Err(Error::SigninError(error::SigninError::PasswordError));
    }
    let public_user = PublicUser::from(user);
    let token = create(public_user, SETTING.auth.secret.as_str())
        .map_err(|_| error::AuthError::AuthCreation)?;
    Ok(token)
}

// test:
// curl -X POST http://127.0.0.1:8080/user/info -H "Content-Type: application/json"
async fn info(claims_user: Option<ClaimsUser>) -> Result<Json<ClaimsUser>, Error> {
    match claims_user {
        Some(a) => {
            let _ = check_claims_user(&a).await?;
            Ok(Json(a))
        },
        None => Err(Error::AuthError(error::AuthError::AuthMiss)),
    }
}
