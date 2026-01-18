#[derive(Debug, thiserror::Error, PartialEq)]
pub enum RepositoryError {
    #[error("Infrastructure error: {0}")]
    Infrastructure(String),
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
