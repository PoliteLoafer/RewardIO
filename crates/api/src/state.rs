use rewardio_core::{AuthService, MessageService};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub message_service: Arc<dyn MessageService>,
    pub auth_service: Arc<dyn AuthService>,
}
