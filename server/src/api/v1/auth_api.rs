use axum::{
    extract::State,
    http::StatusCode,
    routing::post,
    Json, Router,
};

use crate::AppState;

use crate::schemas::test_schema::TestResponse;
use crate::schemas::user_schema::UserResponse;
use crate::schemas::auth_schema::LoginRequest;

use crate::services::auth_service;

pub fn router() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/login", post(login))
        .route("/register", post(register))
}

// Login user
pub async fn login(State(state): State<AppState>, Json(payload): Json<LoginRequest>) -> Result<Json<UserResponse>, StatusCode> {
    match auth_service::login(&state.db_pool, &payload.email, &payload.password) {
        Ok(user) => Ok(Json(user)),
        Err(diesel::result::Error::NotFound) => Err(StatusCode::UNAUTHORIZED),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Register user
pub async fn register(State(_state): State<AppState>) -> Json<TestResponse> {
    Json(auth_service::register())
}
