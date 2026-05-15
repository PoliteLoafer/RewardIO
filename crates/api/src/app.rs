use crate::AppState;
use crate::auth;
use crate::config::Config;
use crate::hello;
use axum::{
    http::{header},
    Router,
    routing::{get, post},
};
use tower_http::cors::CorsLayer;

#[cfg(test)]
use rewardio_core::{RepositoryError, User, UserRepository};

pub struct App {
    pub config: Config,
    pub state: AppState,
}

impl App {
    pub fn new(config: Config, state: AppState) -> Self {
        Self { config, state }
    }

    pub fn router(&self) -> Router {
        Self::router_from_parts(self.state.clone(), &self.config)
    }

    pub fn router_from_state(state: AppState) -> Router {
        let config = Config {
            app_env: "development".to_string(),
            server_port: 3000,
            log_level: "info".to_string(),
            log_to_file: false,
            log_to_console: true,
            log_dir: "logs/back_logs".to_string(),
            postgres_url: "postgres://rewardio:rewardio@localhost:5432/rewardio".to_string(),
            db_connect_max_retries: 10,
            db_connect_retry_delay_secs: 2,
            db_acquire_timeout_secs: 5,
            db_max_connections: 5,
            cors_allowed_origins: vec!["http://localhost:5173".to_string()],
            cors_allowed_methods: vec!["GET".to_string(), "POST".to_string(), "OPTIONS".to_string()],
        };

        Self::router_from_parts(state, &config)
    }

    fn router_from_parts(state: AppState, config: &Config) -> Router {
        let cors_layer = build_cors_layer(config);

        Router::new()
            .route("/api/hello", get(hello::handlers::hello))
            .route("/api/signup", post(auth::handlers::signup))
            .route("/api/signin", post(auth::handlers::signin))
            .layer(cors_layer)
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

fn build_cors_layer(config: &Config) -> CorsLayer {
    let allowed_origins = config
        .parsed_cors_origins()
        .expect("CORS origins must be validated in Config::from_env");
    let allowed_methods = config
        .parsed_cors_methods()
        .expect("CORS methods must be validated in Config::from_env");

    CorsLayer::new()
        .allow_origin(allowed_origins)
        .allow_methods(allowed_methods)
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
        .allow_credentials(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use async_trait::async_trait;
    use rewardio_core::{AuthService, AuthServiceImpl, Message, MessageService, UserRole};
    use tokio::sync::Mutex;

    struct TestMessageService;

    #[async_trait]
    impl MessageService for TestMessageService {
        async fn get_hello_message(&self) -> Result<Message, RepositoryError> {
            Ok(Message {
                message: "Hello from Axum Workspace!".to_string(),
            })
        }
    }

    struct TestUserRepository {
        users: Mutex<Vec<User>>,
    }

    #[async_trait]
    impl UserRepository for TestUserRepository {
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
    async fn test_app_new_and_router() {
        let config = Config {
            app_env: "development".to_string(),
            server_port: 3001,
            log_level: "info".to_string(),
            log_to_file: false,
            log_to_console: false,
            log_dir: "logs/back_logs".to_string(),
            postgres_url: "postgres://rewardio:rewardio@localhost:5432/rewardio".to_string(),
            db_connect_max_retries: 10,
            db_connect_retry_delay_secs: 2,
            db_acquire_timeout_secs: 5,
            db_max_connections: 5,
            cors_allowed_origins: vec!["http://localhost:5173".to_string()],
            cors_allowed_methods: vec!["GET".to_string(), "POST".to_string(), "OPTIONS".to_string()],
        };

        use std::sync::Arc;
        let message_service = Arc::new(TestMessageService) as Arc<dyn rewardio_core::MessageService>;
        let user_repo = Arc::new(TestUserRepository {
            users: Mutex::new(vec![User {
                login: "testuser".to_string(),
                password: "hashed".to_string(),
                name: "Test User".to_string(),
                email: "test@example.com".to_string(),
                role: UserRole::User,
            }]),
        });
        let auth_service = Arc::new(AuthServiceImpl {
            repository: user_repo,
        }) as Arc<dyn AuthService>;

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

        assert_eq!(response.status(), StatusCode::OK);
    }
}
