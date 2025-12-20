use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::QueryResult;

use crate::db::models::user_model::User;
use crate::db::schema::users::dsl:: {users, email};

pub fn get_by_id(conn: &mut PgConnection, user_id: i64) -> QueryResult<User> {
    users.find(user_id).get_result(conn)
}

pub fn get_by_email(conn: &mut PgConnection, user_email: &str) -> QueryResult<User> {
    users.filter(email.eq(user_email)).first(conn)
}
