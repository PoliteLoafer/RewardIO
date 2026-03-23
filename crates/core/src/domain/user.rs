use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum UserRole {
    Admin,
    User,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct User {
    pub login: String,
    pub password: String,
    pub name: String,
    pub email: String,
    pub role: UserRole,
}
