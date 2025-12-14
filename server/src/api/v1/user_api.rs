use axum::{extract::Path, routing::get, Json, Router};

use crate::services::user_service;
use crate::schemas::test_schema::TestResponse;

pub fn router() -> Router {
    Router::new()
        .route("/getbyid/:id", get(get_user_by_id))
}

pub async fn get_user_by_id(Path(id): Path<u64>) -> Json<TestResponse> {
    Json(user_service::get_by_id(id))
}
