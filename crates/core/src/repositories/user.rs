use crate::domain::user::User;
use crate::errors::RepositoryError;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_login(&self, login: &str) -> Result<Option<User>, RepositoryError>;
    async fn save(&self, user: User) -> Result<(), RepositoryError>;
}
