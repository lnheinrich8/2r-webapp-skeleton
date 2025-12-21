use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

pub type UserResult<T> = Result<T, UserError>;

#[derive(Debug)]
pub enum UserError {
    NotFound,
}

#[derive(Serialize)]
struct UserErrorResponse {
    error: String,
}

impl IntoResponse for UserError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            UserError::NotFound => (StatusCode::NOT_FOUND, "User not found"),
        };
        let body = Json(UserErrorResponse {
            error: message.to_string(),
        });
        (status, body).into_response()
    }
}
