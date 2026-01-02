use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::QueryResult;

use crate::db::models::user_model::{NewUser, User};
use crate::db::schema::users::dsl::{email, firstname as firstname_column, lastname as lastname_column, users};

pub fn get_by_id(conn: &mut PgConnection, user_id: i64) -> QueryResult<User> {
    users.find(user_id).get_result(conn)
}

pub fn get_by_email(conn: &mut PgConnection, user_email: &str) -> QueryResult<User> {
    users.filter(email.eq(user_email)).first(conn)
}

pub fn create(conn: &mut PgConnection, new_user: &NewUser) -> QueryResult<User> {
    diesel::insert_into(users).values(new_user).get_result(conn)
}

pub fn update_name(conn: &mut PgConnection, user_id: i64, firstname: &str, lastname: &str) -> QueryResult<User> {
    diesel::update(users.find(user_id))
        .set((
            firstname_column.eq(firstname),
            lastname_column.eq(lastname),
        ))
        .get_result(conn)
}

pub fn update_email(conn: &mut PgConnection, user_id: i64, user_email: &str) -> QueryResult<User> {
    diesel::update(users.find(user_id))
        .set(
            email.eq(user_email)
        )
        .get_result(conn)
}   
