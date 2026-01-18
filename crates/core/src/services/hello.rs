use crate::domain::hello::Message;
use async_trait::async_trait;

#[async_trait]
pub trait MessageService: Send + Sync {
    async fn get_hello_message(&self) -> Message;
}
