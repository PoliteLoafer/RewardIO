use async_trait::async_trait;
use rewardio_core::{HelloRepository, Message, MessageService, RepositoryError};
use std::sync::Arc;

pub struct DbMessageService {
    repository: Arc<dyn HelloRepository>,
}

impl DbMessageService {
    pub fn new(repository: Arc<dyn HelloRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl MessageService for DbMessageService {
    async fn get_hello_message(&self) -> Result<Message, RepositoryError> {
        self.repository.get_hello_message().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rewardio_core::HelloRepository;

    struct FakeHelloRepository;

    #[async_trait]
    impl HelloRepository for FakeHelloRepository {
        async fn get_hello_message(&self) -> Result<Message, RepositoryError> {
            Ok(Message {
                message: "Hello from fake repository".to_string(),
            })
        }
    }

    #[tokio::test]
    async fn test_get_hello_message_from_db_service() {
        let repository = Arc::new(FakeHelloRepository) as Arc<dyn HelloRepository>;
        let service = DbMessageService::new(repository);

        let message = service.get_hello_message().await.unwrap();
        assert_eq!(message.message, "Hello from fake repository");
    }
}
