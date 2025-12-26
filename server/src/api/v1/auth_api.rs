use axum::{
    Json, Router,
    extract::{Extension, Query, State},
    routing::{get, post},
};

use crate::AppState;
use crate::core::auth::Claims;
use crate::core::exceptions::{auth_exceptions::AuthError, user_exceptions::UserError};
use crate::schemas::auth_schema::{LoginRequest, RegisterRequest, RegisterValidateResponse, VerificationQuery};
use crate::schemas::user_schema::UserResponse;
use crate::services::{auth_service, user_service};

pub fn public_router() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/verifyregister", get(verify_register))
}

pub fn protected_router() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/getcurrent", get(get_current))
        .route("/logout", get(logout))
}

// Login user and provide cookie with JWT
pub async fn login(State(state): State<AppState>, Json(payload): Json<LoginRequest>) -> Result<impl axum::response::IntoResponse, AuthError> {
    auth_service::login(&state.db_pool, &state.jwt_secret, &payload.email, &payload.password)
}

// Logout current user
pub async fn logout() -> Result<impl axum::response::IntoResponse, AuthError> {
    auth_service::logout()
}

// Register user
pub async fn register(State(state): State<AppState>, Json(payload): Json<RegisterRequest>) -> Result<Json<RegisterValidateResponse>, AuthError> {
    auth_service::register(&state.db_pool, &state.jwt_email_secret, &payload.firstname, &payload.lastname, &payload.email, &payload.password).await?;

    Ok(Json(RegisterValidateResponse {
        message: "Verification email sent. Please check your inbox.".to_string(),
    }))
}

pub async fn verify_register(State(state): State<AppState>, Query(params): Query<VerificationQuery>) -> Result<Json<UserResponse>, AuthError> {
    auth_service::verify_register(&state.db_pool, &state.jwt_email_secret, &params.token).map(Json)
}

// Get current user (for client authorization)
pub async fn get_current(State(state): State<AppState>, Extension(claims): Extension<Claims>) -> Result<Json<UserResponse>, UserError> {
    user_service::get_by_id(&state.db_pool, claims.sub).map(Json)
}
