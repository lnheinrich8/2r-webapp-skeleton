use chrono::{DateTime, Utc};
use diesel::prelude::*;

use crate::db::schema::users;

// Reading
#[derive(Debug, Clone, Queryable, Identifiable, Selectable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub password: String,
    pub firstname: String,
    pub lastname: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Creating
#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub password: &'a str,
    pub firstname: &'a str,
    pub lastname: &'a str,
}

// Updating
#[derive(AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateUser<'a> {
    pub email: Option<&'a str>,
    pub password: Option<&'a str>,
    pub firstname: Option<&'a str>,
    pub lastname: Option<&'a str>,
}
