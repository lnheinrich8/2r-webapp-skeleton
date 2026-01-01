use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

pub type UserResult<T> = Result<T, UserError>;

#[derive(Debug)]
pub enum UserError {
    NotFound,
    Database,
    Conflict,
    Token,
    Email,
}

#[derive(Serialize)]
struct UserErrorResponse {
    error: String,
}

impl IntoResponse for UserError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            UserError::NotFound => (StatusCode::NOT_FOUND, "User not found"),
            UserError::Database => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
            UserError::Conflict => (StatusCode::CONFLICT, "User already exists"),
            UserError::Token => (StatusCode::INTERNAL_SERVER_ERROR, "Token processing failed"),
            UserError::Email => (StatusCode::INTERNAL_SERVER_ERROR, "Email delivery failed"),
        };
        let body = Json(UserErrorResponse {
            error: message.to_string(),
        });
        (status, body).into_response()
    }
}
