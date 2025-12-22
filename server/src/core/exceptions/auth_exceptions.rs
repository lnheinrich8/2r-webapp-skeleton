use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

pub type AuthResult<T> = Result<T, AuthError>;

#[derive(Debug)]
pub enum AuthError {
    Unauthorized,
    Conflict,
    Database,
    Pool,
    Hash,
    Token,
    Email,
}

#[derive(Serialize)]
struct AuthErrorResponse {
    error: String,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::Unauthorized => (StatusCode::UNAUTHORIZED, "Invalid credentials"),
            AuthError::Conflict => (StatusCode::CONFLICT, "User already exists"),
            AuthError::Hash => (StatusCode::INTERNAL_SERVER_ERROR, "Password processing failed"),
            AuthError::Token => (StatusCode::INTERNAL_SERVER_ERROR, "Token processing failed"),
            AuthError::Email => (StatusCode::INTERNAL_SERVER_ERROR, "Email delivery failed"),
            AuthError::Database | AuthError::Pool => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
        };
        let body = Json(AuthErrorResponse {
            error: message.to_string(),
        });
        (status, body).into_response()
    }
}
