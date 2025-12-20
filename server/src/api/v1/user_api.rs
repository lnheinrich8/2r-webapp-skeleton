use axum::{
    extract::{Path, State},
    routing::get,
    Json, 
    Router
};

use crate::AppState;

use crate::schemas::user_schema::UserResponse;
use crate::services::user_service;

pub fn router() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/getbyid/:id", get(get_user_by_id))
        .route("/getbyemail/:email", get(get_user_by_email))
}

// Get user from db with id
pub async fn get_user_by_id(State(state): State<AppState>, Path(id): Path<i64>) -> Json<UserResponse> {
    Json(user_service::get_by_id(&state.db_pool, id))
}

// Get user from db with email
pub async fn get_user_by_email(State(state): State<AppState>, Path(email): Path<String>) -> Json<UserResponse> {
    Json(user_service::get_by_email(&state.db_pool, &email))
}
