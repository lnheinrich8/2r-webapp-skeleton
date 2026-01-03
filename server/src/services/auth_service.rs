use axum::{
    Json,
    http::{HeaderValue, header},
    response::{IntoResponse, Response},
};
use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::{Duration, Utc};
use diesel::result::Error;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

use crate::core::auth::Claims;
use crate::core::exceptions::auth_exceptions::{AuthError, AuthResult};
use crate::db::connection::PgPool;
use crate::db::models::user_model::NewUser;
use crate::db::repositories::user_repo;
use crate::schemas::auth_schema::{AuthMessageResponse, LoginResponse};
use crate::utils::{emailer, mapper};
use crate::utils::email_templates::{REGISTER_VERIFICATION_HTML, EMAIL_UPDATE_VERIFICATION_HTML};

pub fn login(pool: &PgPool, jwt_secret: &str, email: &str, password: &str) -> AuthResult<Response> {
    let mut conn = pool.get().map_err(|_| AuthError::Pool)?; // try to make connection to the r2d2 pool and propagate upwards if error
    
    let user = user_repo::get_by_email(&mut conn, email).map_err(|err| match err {
        Error::NotFound => AuthError::Unauthorized,
        _ => AuthError::Database,
    })?;

    let is_valid = verify(password, &user.password).map_err(|_| AuthError::Hash)?;
    if !is_valid {
        return Err(AuthError::Unauthorized);
    }

    let user_response = mapper::map_user(user);

    // Creating the cookie with JWT
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(7)) // 7 day expiration
        .ok_or(AuthError::Token)?
        .timestamp() as usize;

    let claims = Claims {
        sub: user_response.id,
        email: user_response.email.clone(),
        exp: expiration,
    };

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_secret.as_bytes())).map_err(|_| AuthError::Token)?;

    let cookie_value = format!("token={}; HttpOnly; Secure; SameSite=Strict; Max-Age={}; Path=/", token, 60 * 60 * 24 * 7);

    let mut response = Json(LoginResponse {
        message: "Login successful".to_string(),
        user: user_response,
    })
    .into_response();

    // Insert the cookie into the response
    response.headers_mut().insert(header::SET_COOKIE, HeaderValue::from_str(&cookie_value).map_err(|_| AuthError::Token)?);

    Ok(response)
}

pub fn logout() -> AuthResult<Response> {
    let mut response = Json(AuthMessageResponse {
        message: "Logged out successfully".to_string(),
    })
    .into_response();

    response
        .headers_mut()
        .insert(header::SET_COOKIE, HeaderValue::from_static("token=; HttpOnly; Secure; SameSite=Strict; Max-Age=0; Path=/"));
    Ok(response)
}

pub async fn register(pool: &PgPool, jwt_email_secret: &str, firstname: &str, lastname: &str, email: &str, password: &str) -> AuthResult<AuthMessageResponse> {
    let mut conn = pool.get().map_err(|_| AuthError::Pool)?;

    match user_repo::get_by_email(&mut conn, email) {
        Ok(_) => return Err(AuthError::Conflict), // user already exists with this email so throw error
        Err(Error::NotFound) => {}                // safe to continue with registration
        Err(_) => return Err(AuthError::Database),
    }

    let hashed_password = hash(password, DEFAULT_COST).map_err(|_| AuthError::Hash)?; // hash with bcrypt

    let expiration = Utc::now() // 1 day verification token expiration
        .checked_add_signed(Duration::days(1))
        .ok_or(AuthError::Token)?
        .timestamp() as usize;

    let claims = emailer::RegisterValidateClaims {
        firstname: firstname.to_string(),
        lastname: lastname.to_string(),
        email: email.to_string(),
        password: hashed_password,
        exp: expiration,
    };

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_email_secret.as_bytes())).map_err(|_| AuthError::Token)?;

    emailer::registration_verification(email, &token).await.map_err(|_| AuthError::Email)?;

    Ok(AuthMessageResponse {
        message: format!("Registration verification email link sent to {}. Please check your inbox.", email),
    })
}

pub fn verify_register(pool: &PgPool, jwt_email_secret: &str, token: &str) -> AuthResult<&'static str> {
    let token_data = decode::<emailer::RegisterValidateClaims>(token, &DecodingKey::from_secret(jwt_email_secret.as_bytes()), &Validation::default()).map_err(|_| AuthError::Token)?;

    let mut conn = pool.get().map_err(|_| AuthError::Pool)?;
    if user_repo::get_by_email(&mut conn, &token_data.claims.email).is_ok() {
        return Err(AuthError::Conflict);
    }

    let new_user = NewUser {
        email: &token_data.claims.email,
        password: &token_data.claims.password,
        firstname: &token_data.claims.firstname,
        lastname: &token_data.claims.lastname,
    };

    user_repo::create(&mut conn, &new_user).map_err(|err| match err {
        Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _) => AuthError::Conflict,
        _ => AuthError::Database,
    })?;

    Ok(REGISTER_VERIFICATION_HTML)
}

pub fn verify_email(pool: &PgPool, jwt_email_secret: &str, token: &str) -> AuthResult<&'static str> {
    let token_data = decode::<emailer::EmailValidateClaims>(token, &DecodingKey::from_secret(jwt_email_secret.as_bytes()), &Validation::default()).map_err(|_| AuthError::Token)?;

    let mut conn = pool.get().map_err(|_| AuthError::Pool)?;

    user_repo::update_email(&mut conn, token_data.claims.id, &token_data.claims.newemail).map_err(|err| match err {
        Error::NotFound => AuthError::NotFound,
        _ => AuthError::Database,
    })?;

    Ok(EMAIL_UPDATE_VERIFICATION_HTML)
}
