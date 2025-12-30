use serde::{Deserialize, Serialize};

use crate::schemas::user_schema::UserResponse;

// PAYLOAD SCHEMAS

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct VerificationQuery {
    pub token: String,
}

// RESPONSE SCHEMAS

#[derive(Serialize)]
pub struct LoginResponse {
    pub message: String,
    pub user: UserResponse,
}

#[derive(Serialize)]
pub struct AuthMessageResponse {
    pub message: String
}
