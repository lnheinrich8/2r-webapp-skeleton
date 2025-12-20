use crate::db::connection::PgPool;
use crate::db::repositories::user_repo;
use crate::schemas::user_schema::UserResponse;

pub fn get_by_id(pool: &PgPool, id: i64) -> UserResponse {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");
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
