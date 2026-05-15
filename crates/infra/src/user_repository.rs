use crate::errors::InfraError;
use async_trait::async_trait;
use rewardio_core::{RepositoryError, User, UserRepository, UserRole};
use sqlx::PgPool;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;

pub struct JsonUserRepository {
    file_path: PathBuf,
    lock: Arc<Mutex<()>>,
}

impl JsonUserRepository {
    pub fn new(file_path: PathBuf) -> Self {
        Self {
            file_path,
            lock: Arc::new(Mutex::new(())),
        }
    }

    async fn load_users(&self) -> Result<Vec<User>, RepositoryError> {
        if !self.file_path.exists() {
            return Ok(vec![]);
        }

        let mut file = File::open(&self.file_path).await.map_err(|e| {
            tracing::error!("Failed to open users file: {}", e);
            RepositoryError::from(InfraError::from(e))
        })?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).await.map_err(|e| {
            tracing::error!("Failed to read users file: {}", e);
            RepositoryError::from(InfraError::from(e))
        })?;

        if contents.trim().is_empty() {
            return Ok(vec![]);
        }

        serde_json::from_str(&contents).map_err(|e| {
            tracing::error!("Failed to parse users JSON: {}", e);
            RepositoryError::from(InfraError::from(e))
        })
    }

    async fn save_users(&self, users: &[User]) -> Result<(), RepositoryError> {
        let contents = serde_json::to_string_pretty(users).map_err(|e| {
            tracing::error!("Failed to serialize users to JSON: {}", e);
            RepositoryError::from(InfraError::from(e))
        })?;
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.file_path)
            .await
            .map_err(|e| {
                tracing::error!("Failed to open users file for writing: {}", e);
                RepositoryError::from(InfraError::from(e))
            })?;

        file.write_all(contents.as_bytes()).await.map_err(|e| {
            tracing::error!("Failed to write users to file: {}", e);
            RepositoryError::from(InfraError::from(e))
        })?;

        file.sync_all().await.map_err(|e| {
            tracing::error!("Failed to sync users file: {}", e);
            RepositoryError::from(InfraError::from(e))
        })?;

        Ok(())
    }
}

pub struct PostgresUserRepository {
    pool: Arc<PgPool>,
}

impl PostgresUserRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    fn map_sqlx_error(error: sqlx::Error) -> RepositoryError {
        if let sqlx::Error::Database(db_error) = &error {
            if db_error.code().as_deref() == Some("23505") {
                return RepositoryError::Conflict(
                    "user with same login or email already exists".to_string(),
                );
            }
        }

        RepositoryError::from(InfraError::from(error))
    }

    fn role_to_db(role: &UserRole) -> &'static str {
        match role {
            UserRole::Admin => "admin",
            UserRole::User => "user",
        }
    }

    fn role_from_db(role: &str) -> Result<UserRole, RepositoryError> {
        match role {
            "admin" => Ok(UserRole::Admin),
            "user" => Ok(UserRole::User),
            other => Err(RepositoryError::from(InfraError::DataMapping(format!(
                "unsupported user role value: {other}"
            )))),
        }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_by_login(&self, login: &str) -> Result<Option<User>, RepositoryError> {
        let row = sqlx::query_as::<_, (String, String, String, String, String)>(
            "SELECT login, password, name, email, role FROM users WHERE login = $1",
        )
        .bind(login)
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(Self::map_sqlx_error)?;

        match row {
            Some((login, password, name, email, role)) => Ok(Some(User {
                login,
                password,
                name,
                email,
                role: Self::role_from_db(&role)?,
            })),
            None => Ok(None),
        }
    }

    async fn save(&self, user: User) -> Result<(), RepositoryError> {
        sqlx::query(
            "INSERT INTO users (login, password, name, email, role)
             VALUES ($1, $2, $3, $4, $5)
             ON CONFLICT (login)
             DO UPDATE SET
               password = EXCLUDED.password,
               name = EXCLUDED.name,
               email = EXCLUDED.email,
               role = EXCLUDED.role",
        )
        .bind(user.login)
        .bind(user.password)
        .bind(user.name)
        .bind(user.email)
        .bind(Self::role_to_db(&user.role))
        .execute(self.pool.as_ref())
        .await
        .map_err(Self::map_sqlx_error)?;

        Ok(())
    }
}

#[async_trait]
impl UserRepository for JsonUserRepository {
    async fn find_by_login(&self, login: &str) -> Result<Option<User>, RepositoryError> {
        tracing::debug!("Finding user by login: {}", login);
        let _guard = self.lock.lock().await;
        let users = self.load_users().await?;
        let user = users.into_iter().find(|u| u.login == login);
        if user.is_some() {
            tracing::debug!("User found: {}", login);
        } else {
            tracing::debug!("User not found: {}", login);
        }
        Ok(user)
    }

    async fn save(&self, user: User) -> Result<(), RepositoryError> {
        tracing::info!("Saving user to JSON repository: {}", user.login);
        let _guard = self.lock.lock().await;
        let mut users = self.load_users().await?;

        if let Some(pos) = users.iter().position(|u| u.login == user.login) {
            tracing::debug!("Updating existing user: {}", user.login);
            users[pos] = user;
        } else {
            tracing::debug!("Adding new user: {}", user.login);
            users.push(user);
        }

        self.save_users(&users).await?;
        tracing::info!("User successfully saved to file");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rewardio_core::UserRole;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_load_users_not_exists() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("non_existent.json");
        let repo = JsonUserRepository::new(file_path);
        let users = repo.load_users().await.unwrap();
        assert!(users.is_empty());
    }

    #[tokio::test]
    async fn test_load_users_empty_file() {
        let temp_file = NamedTempFile::new().unwrap();
        let repo = JsonUserRepository::new(temp_file.path().to_path_buf());
        let users = repo.load_users().await.unwrap();
        assert!(users.is_empty());
    }

    #[tokio::test]
    async fn test_find_by_login_not_found() {
        let temp_file = NamedTempFile::new().unwrap();
        let repo = JsonUserRepository::new(temp_file.path().to_path_buf());
        let user = repo.find_by_login("non_existent").await.unwrap();
        assert!(user.is_none());
    }

    #[tokio::test]
    async fn test_save_users_error() {
        // Create a directory where a file is expected to fail save
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().to_path_buf();
        let repo = JsonUserRepository::new(file_path);

        let user = User {
            login: "test".to_string(),
            password: "password".to_string(),
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            role: UserRole::User,
        };

        let result = repo.save(user).await;
        assert!(result.is_err());
    }
}
