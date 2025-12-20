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
    Router::<AppState>::new().route("/getbyid/:id", get(get_user_by_id))
}

pub async fn get_user_by_id(State(state): State<AppState>, Path(id): Path<i64>) -> Json<UserResponse> {
    Json(user_service::get_by_id(&state.db_pool, id))
}
