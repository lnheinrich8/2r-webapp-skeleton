use axum::{
    Json, Router,
    extract::{Extension, Path, State},
    routing::{get, patch},
};

use crate::AppState;
use crate::core::auth::Claims;
use crate::core::exceptions::user_exceptions::UserError;
use crate::schemas::user_schema::{UpdateUserRequest, UpdateUserEmailRequest, UserResponse, UserMessageResponse};
use crate::services::user_service;

pub fn protected_router() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/getbyid/:id", get(get_user_by_id))
        .route("/getbyemail/:email", get(get_user_by_email))
        .route("/update", patch(update_user))
        .route("/updatemail", patch(update_user_email))
}

// Get user from db with id
pub async fn get_user_by_id(State(state): State<AppState>, Path(id): Path<i64>) -> Result<Json<UserResponse>, UserError> {
    user_service::get_by_id(&state.db_pool, id).map(Json)
}

// Get user from db with email
pub async fn get_user_by_email(State(state): State<AppState>, Path(email): Path<String>) -> Result<Json<UserResponse>, UserError> {
    user_service::get_by_email(&state.db_pool, &email).map(Json)
}

// Update user information and return new user
pub async fn update_user(State(state): State<AppState>, Extension(claims): Extension<Claims>, Json(payload): Json<UpdateUserRequest>) -> Result<Json<UserResponse>, UserError> {
    user_service::update(&state.db_pool, claims.sub, &payload.firstname, &payload.lastname).map(Json)
}

// Sends the API link to the verification handler
pub async fn update_user_email(State(state): State<AppState>, Extension(claims): Extension<Claims>, Json(payload): Json<UpdateUserEmailRequest>) -> Result<Json<UserMessageResponse>, UserError> {
    user_service::update_email(&state.db_pool, &state.jwt_email_secret, claims.sub, &payload.email).await.map(Json)
}
