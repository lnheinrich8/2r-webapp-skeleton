use serde::{Deserialize, Serialize};

// PAYLOAD SCHEMAS

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub firstname: String,
    pub lastname: String
}

// // Unused currently
// #[derive(Deserialize)]
// pub struct UpdateUserEmailRequest {
//     pub email: String
// }

// RESPONSE SCHEMAS

#[derive(Clone, Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub email: String,
    pub firstname: String,
    pub lastname: String,
}

// // Unused currently
// #[derive(Serialize)]
// pub struct UserMessageResponse {
//     pub message: String
// }
