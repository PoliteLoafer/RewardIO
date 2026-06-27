#[derive(Debug, thiserror::Error, PartialEq)]
pub enum RepositoryError {
    #[error("Infrastructure error: {0}")]
    Infrastructure(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Conflict: {0}")]
    Conflict(String),
    #[error("Validation error: {0}")]
    Validation(String),
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum AuthError {
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("User not found")]
    UserNotFound,
    #[error("Invalid password")]
    InvalidPassword,
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),
}
