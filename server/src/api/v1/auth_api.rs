use axum::{Json, Router, extract::State, routing::post};

use crate::AppState;
use crate::core::exceptions::auth_exceptions::AuthError;
use crate::schemas::auth_schema::{LoginRequest, RegisterRequest};
use crate::schemas::user_schema::UserResponse;
use crate::services::auth_service;

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/login", post(login)).route("/register", post(register))
}

pub async fn login(State(state): State<AppState>, Json(payload): Json<LoginRequest>) -> Result<Json<UserResponse>, AuthError> {
    auth_service::login(&state.db_pool, &payload.email, &payload.password).map(Json)
}

pub async fn register(State(state): State<AppState>, Json(payload): Json<RegisterRequest>) -> Result<Json<UserResponse>, AuthError> {
    auth_service::register(&state.db_pool, &payload.firstname, &payload.lastname, &payload.email, &payload.password).map(Json)
}
