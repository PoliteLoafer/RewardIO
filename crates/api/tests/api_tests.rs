use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
    response::Response,
};
use rewardio_api::{App, AppState};
use rewardio_core::{AuthService, AuthServiceImpl, MessageService};
use rewardio_infra::{HardcodedMessageService, JsonUserRepository};
use serde_json::Value;
use std::sync::Arc;
use tower::ServiceExt;

#[tokio::test]
async fn test_signup_and_signin_flow() {
    let message_service = Arc::new(HardcodedMessageService) as Arc<dyn MessageService>;
    let user_repo = Arc::new(JsonUserRepository::new("test_api_users.json".into()));
    let auth_service = Arc::new(AuthServiceImpl {
        repository: user_repo,
    }) as Arc<dyn AuthService>;

    let state = AppState {
        message_service,
        auth_service,
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

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body["login"], "testuser");

    // 3. Signin with invalid password
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

    // 4. Signin with non-existent user
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
    let message_service = Arc::new(HardcodedMessageService) as Arc<dyn MessageService>;
    let user_repo = Arc::new(JsonUserRepository::new("test_hello_users.json".into()));
    let auth_service = Arc::new(AuthServiceImpl {
        repository: user_repo,
    }) as Arc<dyn AuthService>;

    let state = AppState {
        message_service,
        auth_service,
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
