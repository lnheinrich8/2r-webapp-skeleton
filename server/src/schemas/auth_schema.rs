use serde::Deserialize;

// PAYLOAD SCHEMAS

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

// RESPONSE SCHEMAS
