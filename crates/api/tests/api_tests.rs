use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode, header},
    response::Response,
};
use async_trait::async_trait;
use rewardio_api::{App, AppState};
use rewardio_core::{AuthService, AuthServiceImpl, Message, MessageService, RepositoryError};
use rewardio_infra::JsonUserRepository;
use serde_json::Value;
use std::sync::Arc;
use tower::ServiceExt;

struct TestMessageService;

#[async_trait]
impl MessageService for TestMessageService {
    async fn get_hello_message(&self) -> Result<Message, RepositoryError> {
        Ok(Message {
            message: "Hello from Axum Workspace!".to_string(),
        })
    }
}

#[tokio::test]
async fn test_signup_and_signin_flow() {
    let message_service = Arc::new(TestMessageService) as Arc<dyn MessageService>;
    let user_repo = Arc::new(JsonUserRepository::new("test_api_users.json".into()));
    let auth_service = Arc::new(AuthServiceImpl {
        repository: user_repo,
    }) as Arc<dyn AuthService>;

    let state = AppState {
        message_service,
        auth_service,
        session_manager: Arc::new(rewardio_api::auth::session::SessionManager::new(
            "test-secret",
            3600,
            false,
        )),
    };
    let app: Router = App::router_from_state(state);

    // 1. Signup
    let signup_payload = serde_json::json!({
        "login": "testuser",
        "password": "password123",
        "name": "Test User",
        "email": "test@example.com"
    });

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/signup")
                .header("Content-Type", "application/json")
                .body(Body::from(signup_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap() as Response;

    assert_eq!(response.status(), StatusCode::CREATED);

    // 2. Signin
    let signin_payload = serde_json::json!({
        "login": "testuser",
        "password": "password123"
    });

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/signin")
                .header("Content-Type", "application/json")
                .body(Body::from(signin_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap() as Response;

    assert_eq!(response.status(), StatusCode::OK);
    let signin_cookie = response
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .expect("signin should set auth cookie")
        .to_string();

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body["login"], "testuser");

    // 3. Authorized user endpoint
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/me")
                .header("Cookie", signin_cookie.clone())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap() as Response;

    assert_eq!(response.status(), StatusCode::OK);

    // 4. Logout and verify protected endpoint access is revoked
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/logout")
                .header("Cookie", &signin_cookie)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap() as Response;

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    let logout_cookie = response
        .headers()
        .get(header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .expect("logout should clear auth cookie")
        .to_string();
    assert!(logout_cookie.contains("rewardio_auth=;"));
    assert!(logout_cookie.contains("Max-Age=0"));

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/me")
                .header("Cookie", signin_cookie)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap() as Response;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/me")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap() as Response;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    // 5. Signin with invalid password
    let signin_invalid_payload = serde_json::json!({
        "login": "testuser",
        "password": "wrongpassword"
    });

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/signin")
                .header("Content-Type", "application/json")
                .body(Body::from(signin_invalid_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap() as Response;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body["error"], "Invalid password");

    // 6. Signin with non-existent user
    let signin_notfound_payload = serde_json::json!({
        "login": "nonexistent",
        "password": "password123"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/signin")
                .header("Content-Type", "application/json")
                .body(Body::from(signin_notfound_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap() as Response;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body["error"], "User not found");

    // Cleanup
    let _ = std::fs::remove_file("test_api_users.json");
}

#[tokio::test]
async fn test_hello_endpoint() {
    let message_service = Arc::new(TestMessageService) as Arc<dyn MessageService>;
    let user_repo = Arc::new(JsonUserRepository::new("test_hello_users.json".into()));
    let auth_service = Arc::new(AuthServiceImpl {
        repository: user_repo,
    }) as Arc<dyn AuthService>;

    let state = AppState {
        message_service,
        auth_service,
        session_manager: Arc::new(rewardio_api::auth::session::SessionManager::new(
            "test-secret",
            3600,
            false,
        )),
    };
    let app: Router = App::router_from_state(state);

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/hello")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap() as Response;

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body["message"], "Hello from Axum Workspace!");

    let _ = std::fs::remove_file("test_hello_users.json");
}
