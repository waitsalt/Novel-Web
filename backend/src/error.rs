use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
#[error("...")]
pub enum AuthError {
    AuthMiss,
    AuthCreation,
    AuthInvalid,
}

#[derive(Debug, thiserror::Error)]
#[error("...")]
pub enum LoginError {
    UserError,
    PasswordError,
}

#[derive(Debug, thiserror::Error)]
#[error("...")]
pub enum Error {
    AuthError(#[from] AuthError),
    LoginError(#[from] LoginError),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let (status_code, code, message) = match self {
            Error::AuthError(AuthError::AuthCreation) => {
                (StatusCode::BAD_REQUEST, 1001, "wrong in create auth")
            }
            Error::AuthError(AuthError::AuthInvalid) => {
                (StatusCode::BAD_REQUEST, 1002, "invalid auth")
            }
            Error::AuthError(AuthError::AuthMiss) => (StatusCode::BAD_REQUEST, 1003, "miss auth"),

            Error::LoginError(LoginError::UserError) => {
                (StatusCode::BAD_REQUEST, 2001, "no this user")
            }
            Error::LoginError(LoginError::PasswordError) => {
                (StatusCode::BAD_REQUEST, 2002, "password is wrong")
            }

            _ => (StatusCode::NOT_FOUND, 4004, "no define this error"),
        };
        let body = Json(json!({
            "code":code,
            "message": message,
        }));
        (status_code, body).into_response()
    }
}
