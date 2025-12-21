use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::QueryResult;

use crate::db::models::user_model::{NewUser, User};
use crate::db::schema::users::dsl::{email, users};

pub fn get_by_id(conn: &mut PgConnection, user_id: i64) -> QueryResult<User> {
    users.find(user_id).get_result(conn)
}

pub fn get_by_email(conn: &mut PgConnection, user_email: &str) -> QueryResult<User> {
    users.filter(email.eq(user_email)).first(conn)
}

pub fn create(conn: &mut PgConnection, new_user: &NewUser) -> QueryResult<User> {
    diesel::insert_into(users).values(new_user).get_result(conn)
}
