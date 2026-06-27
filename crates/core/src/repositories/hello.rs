use crate::{Message, RepositoryError};
use async_trait::async_trait;

#[async_trait]
pub trait HelloRepository: Send + Sync {
    async fn get_hello_message(&self) -> Result<Message, RepositoryError>;
}
