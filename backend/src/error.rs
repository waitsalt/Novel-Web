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
pub enum Error {
    AuthError(#[from] AuthError),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let (status_code, code, message) = match self {
            Error::AuthError(AuthError::AuthCreation) => {
                (StatusCode::BAD_REQUEST, 4003, "wrong in create auth")
            }
            _ => (StatusCode::NOT_FOUND, 4004, "no define error"),
        };
        let body = Json(json!({
            "code":code,
            "message": message,
        }));
        (status_code, body).into_response()
    }
}
