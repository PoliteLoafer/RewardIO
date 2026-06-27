use rewardio_api::logger;
use rewardio_api::{App, AppState, Config};
use rewardio_core::{AuthService, AuthServiceImpl, MessageService};
use rewardio_infra::{DbMessageService, PostgresHelloRepository, PostgresUserRepository};
use sqlx::migrate::Migrator;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info, warn};

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

async fn connect_postgres_with_retry(config: &Config) -> Result<PgPool, sqlx::Error> {
    let mut last_error: Option<sqlx::Error> = None;

    for attempt in 1..=config.db_connect_max_retries {
        match PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(config.db_acquire_timeout_secs))
            .connect(config.postgres_url.as_str())
            .await
        {
            Ok(pool) => return Ok(pool),
            Err(error) => {
                warn!(
                    attempt,
                    max_retries = config.db_connect_max_retries,
                    "failed to connect to postgres: {error}"
                );
                last_error = Some(error);

                if attempt < config.db_connect_max_retries {
                    tokio::time::sleep(Duration::from_secs(config.db_connect_retry_delay_secs))
                        .await;
                }
            }
        }
    }

    Err(last_error.expect("last_error should be set when retries are exhausted"))
}

async fn init_db(config: &Config) -> Result<PgPool, Box<dyn std::error::Error>> {
    let pool = connect_postgres_with_retry(config).await?;

    info!("running database migrations");
    if let Err(error) = MIGRATOR.run(&pool).await {
        error!("database migrations failed: {error}");
        return Err(error.into());
    }
    info!("database migrations completed successfully");

    Ok(pool)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env()?;
    let _guards = logger::init_logger(&config);

    let pool = Arc::new(init_db(&config).await?);

    let hello_repository = Arc::new(PostgresHelloRepository::new(Arc::clone(&pool)));
    let message_service = Arc::new(DbMessageService::new(hello_repository)) as Arc<dyn MessageService>;
    let user_repo = Arc::new(PostgresUserRepository::new(Arc::clone(&pool)));
    let auth_service = Arc::new(AuthServiceImpl {
        repository: user_repo,
    }) as Arc<dyn AuthService>;

    let state = AppState {
        message_service,
        auth_service,
    };

    let app = App::new(config, state);
    if let Err(e) = app.run().await {
        error!("Application error: {:?}", e);
        return Err(e);
    }

    Ok(())
}
