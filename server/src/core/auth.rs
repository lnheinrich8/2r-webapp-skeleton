use axum::{
    body::Body,
    extract::State,
    http::{HeaderMap, Request, header},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};

use crate::AppState;
use crate::core::exceptions::auth_exceptions::AuthError;
use crate::db::repositories::user_repo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub email: String,
    pub exp: usize,
}

pub async fn verify_session(State(state): State<AppState>, mut request: Request<Body>, next: Next) -> Result<Response, AuthError> {
    let token = extract_token(request.headers()).ok_or(AuthError::Unauthorized)?; // throw unauthorized exception if no token exists
    let token_data = decode::<Claims>(&token, &DecodingKey::from_secret(state.jwt_secret.as_bytes()), &Validation::default())
        .map_err(|_| AuthError::Unauthorized)?;

    // Check if user exists
    let mut conn = state.db_pool.get().map_err(|_| AuthError::Pool)?;
    user_repo::get_by_id(&mut conn, token_data.claims.sub).map_err(|_| AuthError::Unauthorized)?;

    request.extensions_mut().insert(token_data.claims); // store decoded claims in extensions map (can read in handlers or downstream middleware)

    Ok(next.run(request).await) // hand request off to the next service/handler in middleware chain
}

fn extract_token(headers: &HeaderMap) -> Option<String> {
    let cookie_header = headers.get(header::COOKIE)?; // if cookie does not exist, function short circuits and returns None
    let cookie_str = cookie_header.to_str().ok()?;

    // Find token in cookie
    cookie_str.split(';').find_map(|pair| {
        let mut parts = pair.trim().splitn(2, '='); // split into name and value
        match (parts.next(), parts.next()) {
            (Some(name), Some(value)) if name == "token" => Some(value.to_string()),
            _ => None,
        }
    })
}
