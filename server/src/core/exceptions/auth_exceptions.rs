use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub type AuthResult<T> = Result<T, AuthError>;

#[derive(Debug)]
pub enum AuthError {
    Unauthorized,
    Conflict,
    Database,
    Pool,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let status = match self {
            AuthError::Unauthorized => StatusCode::UNAUTHORIZED,
            AuthError::Conflict => StatusCode::CONFLICT,
            AuthError::Database | AuthError::Pool => StatusCode::INTERNAL_SERVER_ERROR,
        };
        status.into_response()
    }
}
