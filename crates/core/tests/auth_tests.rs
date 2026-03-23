use async_trait::async_trait;
use rewardio_core::{
    AuthError, AuthService, AuthServiceImpl, RepositoryError, User, UserRepository, UserRole,
};
use std::sync::Arc;
use tokio::sync::Mutex;

struct MockUserRepository {
    users: Mutex<Vec<User>>,
}

#[async_trait]
impl UserRepository for MockUserRepository {
    async fn find_by_login(&self, login: &str) -> Result<Option<User>, RepositoryError> {
        let users = self.users.lock().await;
        Ok(users.iter().find(|u| u.login == login).cloned())
    }

    async fn save(&self, user: User) -> Result<(), RepositoryError> {
        let mut users = self.users.lock().await;
        if let Some(pos) = users.iter().position(|u| u.login == user.login) {
            users[pos] = user;
        } else {
            users.push(user);
        }
        Ok(())
    }
}

#[tokio::test]
async fn test_sign_up_success() {
    let repo = Arc::new(MockUserRepository {
        users: Mutex::new(vec![]),
    });
    let auth_service = AuthServiceImpl {
        repository: repo.clone(),
    };

    let user = User {
        login: "test".to_string(),
        password: "password".to_string(),
        name: "Test User".to_string(),
        email: "test@example.com".to_string(),
        role: UserRole::User,
    };

    let result = auth_service.sign_up(user.clone()).await;
    assert!(result.is_ok());
    let signed_up_user = result.unwrap();
    assert_eq!(signed_up_user.login, "test");
    assert_ne!(signed_up_user.password, "password"); // Should be hashed

    let saved_user = repo.find_by_login("test").await.unwrap().unwrap();
    assert_eq!(saved_user.password, signed_up_user.password);
}

#[tokio::test]
async fn test_sign_up_already_exists() {
    let user = User {
        login: "test".to_string(),
        password: "password".to_string(),
        name: "Test User".to_string(),
        email: "test@example.com".to_string(),
        role: UserRole::User,
    };
    let repo = Arc::new(MockUserRepository {
        users: Mutex::new(vec![user.clone()]),
    });
    let auth_service = AuthServiceImpl { repository: repo };

    let result = auth_service.sign_up(user).await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), AuthError::UserAlreadyExists);
}

#[tokio::test]
async fn test_sign_in_success() {
    let repo = Arc::new(MockUserRepository {
        users: Mutex::new(vec![]),
    });
    let auth_service = AuthServiceImpl {
        repository: repo.clone(),
    };

    let user = User {
        login: "test".to_string(),
        password: "password".to_string(),
        name: "Test User".to_string(),
        email: "test@example.com".to_string(),
        role: UserRole::User,
    };

    auth_service.sign_up(user).await.unwrap();

    let result = auth_service.sign_in("test", "password").await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().login, "test");
}

#[tokio::test]
async fn test_sign_in_invalid_password() {
    let repo = Arc::new(MockUserRepository {
        users: Mutex::new(vec![]),
    });
    let auth_service = AuthServiceImpl {
        repository: repo.clone(),
    };

    let user = User {
        login: "test".to_string(),
        password: "password".to_string(),
        name: "Test User".to_string(),
        email: "test@example.com".to_string(),
        role: UserRole::User,
    };

    auth_service.sign_up(user).await.unwrap();

    let result = auth_service.sign_in("test", "wrong").await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), AuthError::InvalidPassword);
}
