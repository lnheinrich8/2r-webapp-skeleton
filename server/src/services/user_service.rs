use crate::core::exceptions::user_exceptions::{UserError, UserResult};
use crate::db::connection::PgPool;
use crate::db::repositories::user_repo;
use crate::schemas::user_schema::UserResponse;
use crate::utils::mapper;

pub fn get_by_id(pool: &PgPool, id: i64) -> UserResult<UserResponse> {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    let user = user_repo::get_by_id(&mut conn, id).map_err(|_| UserError::NotFound)?;

    Ok(mapper::map_user(user))
}

pub fn get_by_email(pool: &PgPool, email: &str) -> UserResult<UserResponse> {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
    let user = user_repo::get_by_email(&mut conn, email).map_err(|_| UserError::NotFound)?;

    Ok(mapper::map_user(user))
}
