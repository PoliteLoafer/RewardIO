use rewardio_core::{Message, MessageService};
use async_trait::async_trait;

pub struct HardcodedMessageService;

#[async_trait]
impl MessageService for HardcodedMessageService {
    async fn get_hello_message(&self) -> Message {
        Message {
            message: "Hello from Axum Workspace!".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_hardcoded_message_service() {
        let service = HardcodedMessageService;
        let message = service.get_hello_message().await;
        assert_eq!(message.message, "Hello from Axum Workspace!");
    }
}
