use axum::{Json, Router, extract::State, http::StatusCode, routing::post};

use crate::AppState;
use crate::schemas::auth_schema::{LoginRequest, RegisterRequest};
use crate::schemas::user_schema::UserResponse;
use crate::services::auth_service;

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/login", post(login)).route("/register", post(register))
}

pub async fn login(State(state): State<AppState>, Json(payload): Json<LoginRequest>) -> Result<Json<UserResponse>, StatusCode> {
    match auth_service::login(&state.db_pool, &payload.email, &payload.password) {
        Ok(user) => Ok(Json(user)),
        Err(diesel::result::Error::NotFound) => Err(StatusCode::UNAUTHORIZED),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn register(State(state): State<AppState>, Json(payload): Json<RegisterRequest>) -> Result<Json<UserResponse>, StatusCode> {
    match auth_service::register(&state.db_pool, &payload.firstname, &payload.lastname, &payload.email, &payload.password) {
        Ok(user) => Ok(Json(user)),
        Err(diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _)) => Err(StatusCode::CONFLICT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
