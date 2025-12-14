mod api;
mod schemas;
mod services;

use std::net::SocketAddr;

use axum::http::{HeaderValue, Method};
use axum::Router;
use tower_http::cors::CorsLayer;

use crate::api::v1::user_api;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_methods([Method::GET]).allow_origin(
        "http://localhost:5173"
            .parse::<HeaderValue>()
            .expect("Invalid CORS origin"),
    );

    let app = Router::new()
        .nest("/user", user_api::router())
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 5000));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind port 5000");

    println!("Server running on http://{addr}");
    axum::serve(listener, app).await.expect("Server crashed");
}
