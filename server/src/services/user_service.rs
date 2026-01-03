use axum::{
    Json,
    http::{HeaderValue, header},
    response::{IntoResponse, Response},
};
use diesel::result::Error;
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};

use crate::core::exceptions::user_exceptions::{UserError, UserResult};
use crate::db::connection::PgPool;
use crate::db::repositories::user_repo;
use crate::schemas::user_schema::{UserMessageResponse, UserResponse};
use crate::utils::{emailer, mapper};

pub fn get_by_id(pool: &PgPool, id: i64) -> UserResult<UserResponse> {
    let mut conn = pool.get().map_err(|_| UserError::Database)?;

    let user = user_repo::get_by_id(&mut conn, id).map_err(|err| match err {
        Error::NotFound => UserError::NotFound,
        _ => UserError::Database,
    })?;

    Ok(mapper::map_user(user))
}

pub fn get_by_email(pool: &PgPool, email: &str) -> UserResult<UserResponse> {
    let mut conn = pool.get().map_err(|_| UserError::Database)?;

    let user = user_repo::get_by_email(&mut conn, email).map_err(|err| match err {
        Error::NotFound => UserError::NotFound,
        _ => UserError::Database,
    })?;

    Ok(mapper::map_user(user))
}

pub fn update(pool: &PgPool, id: i64, firstname: &str, lastname: &str) -> UserResult<UserResponse> {
    let mut conn = pool.get().map_err(|_| UserError::Database)?;

    let user = user_repo::update_name(&mut conn, id, firstname, lastname).map_err(|err| match err {
        Error::NotFound => UserError::NotFound,
        _ => UserError::Database,
    })?;

    Ok(mapper::map_user(user))
}

pub async fn update_email(pool: &PgPool, jwt_email_secret: &str, id: i64, email: &str) -> UserResult<UserMessageResponse> {
    let mut conn = pool.get().map_err(|_| UserError::Database)?;

    let user = user_repo::get_by_id(&mut conn, id).map_err(|err| match err {
        Error::NotFound => UserError::NotFound,
        _ => UserError::Database,
    })?;

    match user_repo::get_by_email(&mut conn, email) {
        Ok(_) => return Err(UserError::Conflict), // user already exists with this email so throw error
        Err(Error::NotFound) => {} // safe to continue with updating email
        Err(_) => return Err(UserError::Database),
    }

    let expiration = Utc::now() // 1 day verification token expiration
        .checked_add_signed(Duration::days(1))
        .ok_or(UserError::Token)?
        .timestamp() as usize;

    let claims = emailer::EmailValidateClaims {
        id: id,
        firstname: user.firstname.to_string(),
        lastname: user.lastname.to_string(),
        newemail: email.to_string(),
        exp: expiration,
    };

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_email_secret.as_bytes())).map_err(|_| UserError::Token)?;

    emailer::update_email_verification(email, &token).await.map_err(|_| UserError::Email)?;

    Ok(UserMessageResponse {
        message: format!("Verification email link sent to {}", email),
    })
}

pub fn delete(pool: &PgPool, id: i64) -> UserResult<Response> {
    let mut conn = pool.get().map_err(|_| UserError::Database)?;

    user_repo::delete(&mut conn, id).map_err(|err| match err {
        Error::NotFound => UserError::NotFound,
        _ => UserError::Database
    })?;

    let mut response = Json(UserMessageResponse {
        message: "Logged out successfully".to_string(),
    })
    .into_response();

    response
        .headers_mut()
        .insert(header::SET_COOKIE, HeaderValue::from_static("token=; HttpOnly; Secure; SameSite=Strict; Max-Age=0; Path=/"));

    Ok(response)

    // Ok(UserMessageResponse {
    //     message: "User deleted successfully".to_string(),
    // })
}
