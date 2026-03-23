use async_trait::async_trait;
use rewardio_core::{Message, MessageService};

pub struct HardcodedMessageService;

#[async_trait]
impl MessageService for HardcodedMessageService {
    async fn get_hello_message(&self) -> Message {
        tracing::debug!("Generating hello message from HardcodedMessageService");
        Message {
            message: "Hello from Axum Workspace!".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_hello_message() {
        let service = HardcodedMessageService;
        let message = service.get_hello_message().await;
        assert_eq!(message.message, "Hello from Axum Workspace!");
    }
}
