use crate::db::models::user_model::User;
use crate::schemas::user_schema::UserResponse;

pub fn map_user(user: User) -> UserResponse {
    UserResponse {
        id: user.id,
        email: user.email,
        firstname: user.firstname,
        lastname: user.lastname,
    }
}
