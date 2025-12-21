use axum::{
    Json, Router,
    extract::{Extension, State},
    routing::{get, post},
};

use crate::AppState;
use crate::core::auth::Claims;
use crate::core::exceptions::{auth_exceptions::AuthError, user_exceptions::UserError};
use crate::schemas::auth_schema::{LoginRequest, RegisterRequest};
use crate::schemas::user_schema::UserResponse;
use crate::services::{auth_service, user_service};

pub fn public_router() -> Router<AppState> {
    Router::<AppState>::new().route("/login", post(login)).route("/register", post(register))
}

pub fn protected_router() -> Router<AppState> {
    Router::<AppState>::new().route("/getcurrent", get(get_current))
}

pub async fn login(State(state): State<AppState>, Json(payload): Json<LoginRequest>) -> Result<impl axum::response::IntoResponse, AuthError> {
    auth_service::login(&state.db_pool, &state.jwt_secret, &payload.email, &payload.password)
}

pub async fn register(State(state): State<AppState>, Json(payload): Json<RegisterRequest>) -> Result<Json<UserResponse>, AuthError> {
    auth_service::register(&state.db_pool, &payload.firstname, &payload.lastname, &payload.email, &payload.password).map(Json)
}

pub async fn get_current(State(state): State<AppState>, Extension(claims): Extension<Claims>) -> Result<Json<UserResponse>, UserError> {
    user_service::get_by_id(&state.db_pool, claims.sub).map(Json)
}
