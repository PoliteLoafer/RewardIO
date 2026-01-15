use axum::{
    routing::get,
    Json, Router, extract::State,
};
use tower_http::cors::CorsLayer;
use std::net::SocketAddr;
use std::sync::Arc;
use rewardio_core::{Message, MessageService};
use rewardio_infra::HardcodedMessageService;

#[tokio::main]
async fn main() {
    let service = Arc::new(HardcodedMessageService) as Arc<dyn MessageService>;
    let app = app(service);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub fn app(service: Arc<dyn MessageService>) -> Router {
    Router::new()
        .route("/api/hello", get(hello))
        .layer(CorsLayer::permissive())
        .with_state(service)
}

async fn hello(
    State(service): State<Arc<dyn MessageService>>,
) -> Json<Message> {
    Json(service.get_hello_message().await)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;
    use serde_json::Value;

    #[tokio::test]
    async fn test_hello_endpoint() {
        let service = Arc::new(HardcodedMessageService) as Arc<dyn MessageService>;
        let app = app(service);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/hello")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body, serde_json::json!({ "message": "Hello from Axum Workspace!" }));
    }
}
