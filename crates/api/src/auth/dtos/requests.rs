use rewardio_core::{User, UserRole};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SignupRequest {
    pub login: String,
    pub password: String,
    pub name: String,
    pub email: String,
}

impl SignupRequest {
    pub fn into_user(self) -> User {
        User {
            login: self.login,
            password: self.password,
            name: self.name,
            email: self.email,
            role: UserRole::User,
        }
    }
}

#[derive(Deserialize)]
pub struct SigninRequest {
    pub login: String,
    pub password: String,
}
