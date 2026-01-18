use crate::AppState;
use crate::auth;
use crate::config::Config;
use crate::hello;
use axum::{
    Router,
    routing::{get, post},
};
use tower_http::cors::CorsLayer;

pub struct App {
    pub config: Config,
    pub state: AppState,
}

impl App {
    pub fn new(config: Config, state: AppState) -> Self {
        Self { config, state }
    }

    pub fn router(&self) -> Router {
        Self::router_from_state(self.state.clone())
    }

    pub fn router_from_state(state: AppState) -> Router {
        Router::new()
            .route("/api/hello", get(hello::handlers::hello))
            .route("/api/signup", post(auth::handlers::signup))
            .route("/api/signin", post(auth::handlers::signin))
            .layer(CorsLayer::permissive())
            .with_state(state)
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = std::net::SocketAddr::from(([0, 0, 0, 0], self.config.server_port));

        tracing::info!(addr = %addr, "listening on server");

        let app = self.router().layer(
            tower_http::trace::TraceLayer::new_for_http()
                .make_span_with(|request: &axum::http::Request<axum::body::Body>| {
                    tracing::span!(
                        tracing::Level::INFO,
                        "request",
                        method = request.method().as_str(),
                        uri = request.uri().path()
                    )
                })
                .on_request(
                    |_request: &axum::http::Request<axum::body::Body>, _span: &tracing::Span| {
                        tracing::info!("started processing request");
                    },
                )
                .on_response(
                    |response: &axum::http::Response<_>,
                     latency: std::time::Duration,
                     _span: &tracing::Span| {
                        let status = response.status();
                        tracing::info!(
                            latency = ?latency,
                            status = %status.as_u16(),
                            "finished processing request"
                        );
                    },
                ),
        );
        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[tokio::test]
    async fn test_app_new_and_router() {
        let config = Config {
            server_port: 3001,
            log_level: "info".to_string(),
            log_to_file: false,
            log_to_console: false,
            log_dir: "logs/back_logs".to_string(),
            database_path: "test_app_users.json".to_string(),
        };

        use std::sync::Arc;
        let message_service = Arc::new(rewardio_infra::HardcodedMessageService)
            as Arc<dyn rewardio_core::MessageService>;
        let user_repo = Arc::new(rewardio_infra::JsonUserRepository::new(
            config.database_path.clone().into(),
        ));
        let auth_service = Arc::new(rewardio_core::AuthServiceImpl {
            repository: user_repo,
        }) as Arc<dyn rewardio_core::AuthService>;

        let state = AppState {
            message_service,
            auth_service,
        };

        let app_obj = App::new(config, state);
        assert_eq!(app_obj.config.server_port, 3001);

        let router = app_obj.router();

        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use tower::ServiceExt;

        let response = router
            .clone()
            .oneshot(
                Request::builder()
                    .method("GET")
                    .uri("/api/hello")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let _ = std::fs::remove_file("test_app_users.json");
        assert_eq!(response.status(), StatusCode::OK);
    }
}
