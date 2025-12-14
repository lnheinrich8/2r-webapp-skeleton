use serde::Serialize;

// PAYLOAD SCHEMAS

// RESPONSE SCHEMAS

#[derive(Serialize)]
pub struct TestResponse {
    pub message: String,
}
