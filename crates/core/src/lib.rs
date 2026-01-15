use serde::Serialize;
use async_trait::async_trait;

#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct Message {
    pub message: String,
}

#[async_trait]
pub trait MessageService: Send + Sync {
    async fn get_hello_message(&self) -> Message;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_message_struct() {
        let msg = Message { message: "test".to_string() };
        assert_eq!(msg.message, "test");
    }
}
