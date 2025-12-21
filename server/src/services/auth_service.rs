use diesel::result::Error;

use crate::core::exceptions::auth_exceptions::{AuthError, AuthResult};
use crate::db::connection::PgPool;
use crate::db::models::user_model::NewUser;
use crate::db::repositories::user_repo;
use crate::schemas::user_schema::UserResponse;
use crate::utils::mapper;

pub fn login(pool: &PgPool, email: &str, password: &str) -> AuthResult<UserResponse> {
    let mut conn = pool.get().map_err(|_| AuthError::Pool)?; // try to make connection to the r2d2 pool and propagate upwards if error
    let user = user_repo::get_by_email_and_password(&mut conn, email, password).map_err(|err| match err {
        Error::NotFound => AuthError::Unauthorized,
        _ => AuthError::Database,
    })?;

    Ok(mapper::map_user(user))
}

pub fn register(pool: &PgPool, firstname: &str, lastname: &str, email: &str, password: &str) -> AuthResult<UserResponse> {
    let mut conn = pool.get().map_err(|_| AuthError::Pool)?;
    let new_user = NewUser {
        email,
        password,
        firstname,
        lastname,
    };
    let user = user_repo::create(&mut conn, &new_user).map_err(|err| match err {
        Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _) => AuthError::Conflict,
        _ => AuthError::Database,
    })?;

    Ok(mapper::map_user(user))
}
