use crate::db::models::user_model::User;
use crate::schemas::user_schema::UserResponse;

pub fn map_user(user: User) -> UserResponse {
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
