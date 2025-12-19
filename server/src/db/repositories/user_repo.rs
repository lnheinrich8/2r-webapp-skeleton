use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::QueryResult;

use crate::db::models::user_model::User;
use crate::db::schema::users::dsl::users;

pub fn get_by_id(conn: &mut PgConnection, user_id: i64) -> QueryResult<User> {
    users.find(user_id).get_result(conn)
}
