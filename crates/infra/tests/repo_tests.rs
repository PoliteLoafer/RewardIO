use rewardio_core::{User, UserRepository, UserRole};
use rewardio_infra::JsonUserRepository;
use tempfile::NamedTempFile;

#[tokio::test]
async fn test_json_repo_save_and_find() {
    let temp_file = NamedTempFile::new().unwrap();
    let file_path = temp_file.path().to_path_buf();
    let repo = JsonUserRepository::new(file_path);

    let user = User {
        login: "test".to_string(),
        password: "password".to_string(),
        name: "Test User".to_string(),
        email: "test@example.com".to_string(),
        role: UserRole::User,
    };

    let save_result = repo.save(user.clone()).await;
    assert!(save_result.is_ok());

    let found_user = repo.find_by_login("test").await.unwrap();
    assert!(found_user.is_some());
    assert_eq!(found_user.unwrap().login, "test");
}

#[tokio::test]
async fn test_json_repo_update() {
    let temp_file = NamedTempFile::new().unwrap();
    let file_path = temp_file.path().to_path_buf();
    let repo = JsonUserRepository::new(file_path);

    let user = User {
        login: "test".to_string(),
        password: "password".to_string(),
        name: "Test User".to_string(),
        email: "test@example.com".to_string(),
        role: UserRole::User,
    };

    repo.save(user.clone()).await.unwrap();

    let mut updated_user = user.clone();
    updated_user.name = "Updated Name".to_string();
    repo.save(updated_user).await.unwrap();

    let found_user = repo.find_by_login("test").await.unwrap().unwrap();
    assert_eq!(found_user.name, "Updated Name");
}
