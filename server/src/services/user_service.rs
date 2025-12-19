use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use crate::db::repositories::user_repo;
use crate::schemas::user_schema::UserResponse;

pub fn get_by_id(id: i64) -> UserResponse {
    let mut conn = establish_connection();
    let user = user_repo::get_by_id(&mut conn, id).expect("User not found");

    UserResponse {
        id: user.id,
        email: user.email,
        password: user.password,
        firstname: user.firstname,
        lastname: user.lastname,
        created_at: user.created_at,
        updated_at: user.updated_at,
    }
}

fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect("Failed to connect to database")
}
