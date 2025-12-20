use diesel::result::Error;

use crate::db::connection::PgPool;
use crate::db::repositories::user_repo;

use crate::schemas::test_schema::TestResponse;
use crate::schemas::user_schema::UserResponse;

pub fn login(pool: &PgPool, email: &str, password: &str) -> Result<UserResponse, Error> {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    let user = user_repo::get_by_email_and_password(&mut conn, email, password)?;

    Ok(UserResponse {
        id: user.id,
        email: user.email,
        password: user.password,
        firstname: user.firstname,
        lastname: user.lastname,
        created_at: user.created_at,
        updated_at: user.updated_at,
    })
}

pub fn register() -> TestResponse {
    TestResponse {
        message: format!("Temp register response"),
    }
}
