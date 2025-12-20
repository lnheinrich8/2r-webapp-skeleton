mod api;
mod core;
mod db;
mod schemas;
mod services;
mod utils;

use std::{env, net::SocketAddr};

use axum::http::{header::CONTENT_TYPE, HeaderValue, Method};
use axum::Router;
use dotenvy::dotenv;
use tower_http::cors::CorsLayer;

use crate::api::v1::{auth_api, user_api};
use crate::db::connection::{init_pool, PgPool};

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = init_pool(&database_url);
    let state = AppState { db_pool: pool };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([CONTENT_TYPE])
        .allow_origin(
            "http://localhost:5173"
                .parse::<HeaderValue>()
                .expect("Invalid CORS origin"),
        );

    let app = Router::<AppState>::new()
        .nest("/user", user_api::router())
        .nest("/auth", auth_api::router())
        .layer(cors)
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 5000));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind port 5000");

    println!("Server running on http://{addr}");
    axum::serve(listener, app).await.expect("Server crashed");
}
