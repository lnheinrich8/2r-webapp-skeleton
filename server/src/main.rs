mod api;
mod core;
mod db;
mod schemas;
mod services;
mod utils;

use std::{env, net::SocketAddr};

use axum::{
    Router,
    http::{HeaderValue, Method, header::CONTENT_TYPE},
    middleware,
};
use dotenvy::dotenv;
use tower_http::cors::CorsLayer;

use crate::api::v1::{auth_api, user_api};
use crate::core::auth;
use crate::db::connection::{PgPool, init_pool};

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub jwt_secret: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = init_pool(&database_url);
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let state = AppState { db_pool: pool, jwt_secret };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([CONTENT_TYPE])
        .allow_credentials(true)
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().expect("Invalid CORS origin"));

    // Routers
    let user_protected = user_api::router().route_layer(middleware::from_fn_with_state(state.clone(), auth::verify_session));
    let auth_public = auth_api::public_router();
    let auth_protected = auth_api::protected_router().route_layer(middleware::from_fn_with_state(state.clone(), auth::verify_session));

    let app = Router::<AppState>::new()
        .nest("/user", user_protected)
        .nest("/auth", auth_public)
        .nest("/auth", auth_protected)
        .layer(cors)
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 5000));
    let listener = tokio::net::TcpListener::bind(addr).await.expect("Failed to bind port 5000");

    println!("Server running on http://{addr}");
    axum::serve(listener, app).await.expect("Server crashed");
}
