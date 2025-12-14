use crate::schemas::test_schema::TestResponse;

pub fn get_by_id(id: u64) -> TestResponse {
    TestResponse {
        message: format!("Temp getuserbyid response for id {}", id),
    }
}
