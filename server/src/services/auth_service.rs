use crate::schemas::test_schema::TestResponse;

pub fn register() -> TestResponse {
    TestResponse {
        message: format!("Temp register response"),
    }
}
