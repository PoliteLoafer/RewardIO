use async_trait::async_trait;
use crate::errors::InfraError;
use rewardio_core::{HelloRepository, Message, RepositoryError};
use sqlx::PgPool;
use std::sync::Arc;

pub struct PostgresHelloRepository {
    pool: Arc<PgPool>,
}

impl PostgresHelloRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl HelloRepository for PostgresHelloRepository {
    async fn get_hello_message(&self) -> Result<Message, RepositoryError> {
        let message = sqlx::query_scalar::<_, String>(
            "SELECT message FROM hello_messages ORDER BY id ASC LIMIT 1",
        )
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(|error| RepositoryError::from(InfraError::from(error)))?;

        match message {
            Some(message) => Ok(Message { message }),
            None => Err(RepositoryError::NotFound(
                "hello_messages table is empty".to_string(),
            )),
        }
    }
}
