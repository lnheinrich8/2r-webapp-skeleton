use axum::{
    extract::State,
    routing::get,
    Json, 
    Router
};

use crate::AppState;

use crate::schemas::test_schema::TestResponse;
use crate::services::auth_service;

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/register", get(register))
}

pub async fn register(State(_state): State<AppState>) -> Json<TestResponse> {
    Json(auth_service::register())
}
