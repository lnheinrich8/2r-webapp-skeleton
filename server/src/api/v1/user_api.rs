use axum::{extract::Path, routing::get, Json, Router};

use crate::schemas::user_schema::UserResponse;
use crate::services::user_service;

pub fn router() -> Router {
    Router::new()
        .route("/getbyid/:id", get(get_user_by_id))
}

pub async fn get_user_by_id(Path(id): Path<i64>) -> Json<UserResponse> {
    Json(user_service::get_by_id(id))
}
