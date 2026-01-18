use rewardio_core::{User, UserRole};
use serde::Serialize;

#[derive(Serialize)]
pub struct UserResponse {
    pub login: String,
    pub name: String,
    pub email: String,
    pub role: UserRole,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            login: user.login,
            name: user.name,
            email: user.email,
            role: user.role,
        }
    }
}
