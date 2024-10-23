use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
#[error("...")]
pub enum AuthError {
    AuthMiss,
    AuthCreation,
    AuthInvalid,
    AuthTimeout,
}

#[derive(Debug, thiserror::Error)]
#[error("...")]
pub enum SigninError {
    UserError,
    PasswordError,
}

#[derive(Debug, thiserror::Error)]
#[error("...")]
pub enum SignupError {
    UserExist,
    QueryError,
    EmailExist,
}

#[derive(Debug, thiserror::Error)]
#[error("...")]
pub enum Error {
    AuthError(#[from] AuthError),
    SigninError(#[from] SigninError),
    SignupError(#[from] SignupError),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let (status_code, code, message) = match self {
            // Auth
            Error::AuthError(AuthError::AuthCreation) => {
                (StatusCode::BAD_REQUEST, 1001, "wrong in create auth")
            }
            Error::AuthError(AuthError::AuthInvalid) => {
                (StatusCode::BAD_REQUEST, 1002, "invalid auth")
            }
            Error::AuthError(AuthError::AuthMiss) => (StatusCode::BAD_REQUEST, 1003, "miss auth"),
            Error::AuthError(AuthError::AuthTimeout) => {
                (StatusCode::BAD_REQUEST, 1004, "auth is time out")
            }

            // Signin
            Error::SigninError(SigninError::UserError) => {
                (StatusCode::BAD_REQUEST, 2001, "no this user")
            }
            Error::SigninError(SigninError::PasswordError) => {
                (StatusCode::BAD_REQUEST, 2002, "password is wrong")
            }

            // Signup
            Error::SignupError(SignupError::UserExist) => {
                (StatusCode::BAD_REQUEST, 3001, "user already exist")
            }
            Error::SignupError(SignupError::QueryError) => {
                (StatusCode::BAD_REQUEST, 3002, "query error")
            }
            Error::SignupError(SignupError::EmailExist) => {
                (StatusCode::BAD_REQUEST, 3003, "email already exist")
            }
        };
        let body = Json(json!({
            "code":code,
            "message": message,
        }));
        (status_code, body).into_response()
    }
}
