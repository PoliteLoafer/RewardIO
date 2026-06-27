use rewardio_core::RepositoryError;

#[derive(Debug, thiserror::Error)]
pub enum InfraError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("data mapping error: {0}")]
    DataMapping(String),
}

impl From<InfraError> for RepositoryError {
    fn from(value: InfraError) -> Self {
        match value {
            InfraError::DataMapping(message) => RepositoryError::Validation(message),
            other => RepositoryError::Infrastructure(other.to_string()),
        }
    }
}