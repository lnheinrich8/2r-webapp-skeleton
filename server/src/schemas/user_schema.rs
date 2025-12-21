use serde::Serialize;

// PAYLOAD SCHEMAS

// RESPONSE SCHEMAS

#[derive(Clone, Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub email: String,
    pub firstname: String,
    pub lastname: String,
}
