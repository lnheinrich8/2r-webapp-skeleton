use axum::{routing::get, Json, Router};

use crate::services::auth_service;
use crate::schemas::test_schema::TestResponse;

pub fn router() -> Router {
    Router::new()
        .route("/register", get(register))
}

pub async fn register() -> Json<TestResponse> {
    Json(auth_service::register())
}
