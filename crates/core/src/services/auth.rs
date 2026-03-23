use crate::domain::user::User;
use crate::errors::AuthError;
use crate::repositories::user::UserRepository;
use async_trait::async_trait;
use bcrypt::{DEFAULT_COST, hash, verify};
use std::sync::Arc;

#[async_trait]
pub trait AuthService: Send + Sync {
    async fn sign_up(&self, user: User) -> Result<User, AuthError>;
    async fn sign_in(&self, login: &str, password: &str) -> Result<User, AuthError>;
}

pub struct AuthServiceImpl {
    pub repository: Arc<dyn UserRepository>,
}

#[async_trait]
impl AuthService for AuthServiceImpl {
    async fn sign_up(&self, mut user: User) -> Result<User, AuthError> {
        tracing::info!("Attempting to sign up user: {}", user.login);
        if self.repository.find_by_login(&user.login).await?.is_some() {
            tracing::warn!("Sign up failed: User {} already exists", user.login);
            return Err(AuthError::UserAlreadyExists);
        }

        let hashed_password = hash(&user.password, DEFAULT_COST).map_err(|e| {
            tracing::error!("Failed to hash password: {}", e);
            AuthError::Repository(crate::errors::RepositoryError::Infrastructure(
                e.to_string(),
            ))
        })?;
        user.password = hashed_password;

        let saved_user = user.clone();
        self.repository.save(user).await?;
        tracing::info!("User successfully signed up");
        Ok(saved_user)
    }

    async fn sign_in(&self, login: &str, password: &str) -> Result<User, AuthError> {
        tracing::info!("Attempting to sign in user: {}", login);
        let user = self.repository.find_by_login(login).await?.ok_or_else(|| {
            tracing::warn!("Sign in failed: User {} not found", login);
            AuthError::UserNotFound
        })?;

        let is_valid = verify(password, &user.password).map_err(|e| {
            tracing::error!("Failed to verify password: {}", e);
            AuthError::Repository(crate::errors::RepositoryError::Infrastructure(
                e.to_string(),
            ))
        })?;

        if !is_valid {
            tracing::warn!("Sign in failed: Invalid password for user {}", login);
            return Err(AuthError::InvalidPassword);
        }

        tracing::info!("User {} successfully signed in", login);
        Ok(user)
    }
}
