use rewardio_api::logger;
use rewardio_api::{App, AppState, Config};
use rewardio_core::{AuthService, AuthServiceImpl, MessageService};
use rewardio_infra::{HardcodedMessageService, JsonUserRepository};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env()?;
    let _guards = logger::init_logger(&config);

    let message_service = Arc::new(HardcodedMessageService) as Arc<dyn MessageService>;
    let user_repo = Arc::new(JsonUserRepository::new(config.database_path.clone().into()));
    let auth_service = Arc::new(AuthServiceImpl {
        repository: user_repo,
    }) as Arc<dyn AuthService>;

    let state = AppState {
        message_service,
        auth_service,
    };

    let app = App::new(config, state);
    app.run().await
}
