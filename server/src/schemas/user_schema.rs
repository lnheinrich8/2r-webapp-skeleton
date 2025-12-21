use chrono::{DateTime, Utc};
use serde::Serialize;

// PAYLOAD SCHEMAS

// RESPONSE SCHEMAS

#[derive(Clone, Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub email: String,
    pub password: String,
    pub firstname: String,
    pub lastname: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
