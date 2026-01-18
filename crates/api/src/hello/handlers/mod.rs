use crate::state::AppState;
use axum::{Json, extract::State};
use rewardio_core::Message;

pub async fn hello(State(state): State<AppState>) -> Json<Message> {
    Json(state.message_service.get_hello_message().await)
}
