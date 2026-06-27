use crate::state::AppState;
use axum::{Json, extract::State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub async fn hello(State(state): State<AppState>) -> Response {
    match state.message_service.get_hello_message().await {
        Ok(message) => (StatusCode::OK, Json(message)).into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": error.to_string() })),
        )
            .into_response(),
    }
}
