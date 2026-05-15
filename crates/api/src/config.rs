use axum::http::{HeaderValue, Method};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub app_env: String,
    pub server_port: u16,
    pub log_level: String,
    pub log_to_file: bool,
    pub log_to_console: bool,
    pub log_dir: String,
    pub postgres_url: String,
    pub db_connect_max_retries: usize,
    pub db_connect_retry_delay_secs: u64,
    pub db_acquire_timeout_secs: u64,
    pub db_max_connections: u32,
    pub cors_allowed_origins: Vec<String>,
    pub cors_allowed_methods: Vec<String>,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        dotenvy::dotenv().ok();

        let app_env = std::env::var("APP_ENV")
            .or_else(|_| std::env::var("REWARDIO__APP_ENV"))
            .unwrap_or_else(|_| "development".to_string());
        let is_production = matches!(app_env.as_str(), "production" | "prod");

        let mut builder = config::Config::builder()
            .set_default("app_env", app_env)?
            .set_default("server_port", 3000)?
            .set_default("log_level", "info")?
            .set_default("log_to_file", false)?
            .set_default("log_to_console", true)?
            .set_default("log_dir", "logs/back_logs")?
            .set_default("db_connect_max_retries", 10)?
            .set_default("db_connect_retry_delay_secs", 2)?
            .set_default("db_acquire_timeout_secs", 5)?
            .set_default("db_max_connections", 5)?
            .set_default("cors_allowed_methods", vec!["GET", "POST", "OPTIONS"])?
            .add_source(
                config::Environment::default()
                    .list_separator(",")
                    .with_list_parse_key("cors_allowed_origins")
                    .with_list_parse_key("cors_allowed_methods"),
            )
            .add_source(
                config::Environment::with_prefix("REWARDIO")
                    .separator("__")
                    .list_separator(",")
                    .with_list_parse_key("cors_allowed_origins")
                    .with_list_parse_key("cors_allowed_methods"),
            );

        if !is_production {
            builder = builder.set_default(
                "postgres_url",
                "postgres://rewardio:rewardio@localhost:5432/rewardio",
            )?
            .set_default("cors_allowed_origins", vec!["http://localhost:5173"])?;
        }

        let config: Self = builder.build()?.try_deserialize()?;
        config.validate()?;
        Ok(config)
    }

    pub fn is_production(&self) -> bool {
        matches!(self.app_env.as_str(), "production" | "prod")
    }

    pub fn parsed_cors_origins(&self) -> Result<Vec<HeaderValue>, config::ConfigError> {
        self.cors_allowed_origins
            .iter()
            .map(|origin| {
                HeaderValue::from_str(origin.trim()).map_err(|error| {
                    config::ConfigError::Message(format!(
                        "invalid CORS origin '{origin}': {error}"
                    ))
                })
            })
            .collect()
    }

    pub fn parsed_cors_methods(&self) -> Result<Vec<Method>, config::ConfigError> {
        self.cors_allowed_methods
            .iter()
            .map(|method| {
                let normalized = method.trim().to_uppercase();
                Method::from_bytes(normalized.as_bytes()).map_err(|error| {
                    config::ConfigError::Message(format!(
                        "invalid CORS method '{method}': {error}"
                    ))
                })
            })
            .collect()
    }

    fn validate(&self) -> Result<(), config::ConfigError> {
        if self.db_connect_max_retries == 0 {
            return Err(config::ConfigError::Message(
                "db_connect_max_retries must be greater than 0".to_string(),
            ));
        }

        if self.db_acquire_timeout_secs == 0 {
            return Err(config::ConfigError::Message(
                "db_acquire_timeout_secs must be greater than 0".to_string(),
            ));
        }

        if self.db_max_connections == 0 {
            return Err(config::ConfigError::Message(
                "db_max_connections must be greater than 0".to_string(),
            ));
        }

        if self.cors_allowed_origins.is_empty() {
            return Err(config::ConfigError::Message(
                "cors_allowed_origins must contain at least one origin".to_string(),
            ));
        }

        if self.cors_allowed_methods.is_empty() {
            return Err(config::ConfigError::Message(
                "cors_allowed_methods must contain at least one method".to_string(),
            ));
        }

        if self.is_production() && self.cors_allowed_origins.iter().any(|origin| origin == "*") {
            return Err(config::ConfigError::Message(
                "wildcard CORS origin '*' is not allowed in production".to_string(),
            ));
        }

        let _ = self.parsed_cors_origins()?;
        let _ = self.parsed_cors_methods()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_config_defaults() {
        let builder = config::Config::builder()
            .set_default("app_env", "development")
            .unwrap()
            .set_default("server_port", 3000)
            .unwrap()
            .set_default("log_level", "info")
            .unwrap()
            .set_default("log_to_file", false)
            .unwrap()
            .set_default("log_to_console", true)
            .unwrap()
            .set_default("log_dir", "logs/back_logs")
            .unwrap()
            .set_default("db_connect_max_retries", 10)
            .unwrap()
            .set_default("db_connect_retry_delay_secs", 2)
            .unwrap()
            .set_default("db_acquire_timeout_secs", 5)
            .unwrap()
            .set_default("db_max_connections", 5)
            .unwrap()
            .set_default("cors_allowed_origins", vec!["http://localhost:5173"])
            .unwrap()
            .set_default("cors_allowed_methods", vec!["GET", "POST", "OPTIONS"])
            .unwrap()
            .set_default(
                "postgres_url",
                "postgres://rewardio:rewardio@localhost:5432/rewardio",
            )
            .unwrap();

        let config: Config = builder.build().unwrap().try_deserialize().unwrap();
        assert_eq!(config.server_port, 3000);
        assert_eq!(config.log_level, "info");
        assert_eq!(config.db_connect_max_retries, 10);
        assert_eq!(config.cors_allowed_origins, vec!["http://localhost:5173"]);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_override() {
        unsafe {
            env::set_var("REWARDIO_TEST__SERVER_PORT", "4000");
            env::set_var("REWARDIO_TEST__LOG_LEVEL", "debug");
            env::set_var("REWARDIO_TEST__DB_CONNECT_MAX_RETRIES", "7");
            env::set_var("REWARDIO_TEST__CORS_ALLOWED_ORIGINS__0", "https://example.com");
            env::set_var(
                "REWARDIO_TEST__CORS_ALLOWED_ORIGINS__1",
                "https://admin.example.com",
            );
        }

        let builder = config::Config::builder()
            .set_default("app_env", "development")
            .unwrap()
            .set_default("server_port", 3000)
            .unwrap()
            .set_default("log_level", "info")
            .unwrap()
            .set_default("log_to_file", false)
            .unwrap()
            .set_default("log_to_console", true)
            .unwrap()
            .set_default("log_dir", "logs/back_logs")
            .unwrap()
            .set_default("db_connect_max_retries", 10)
            .unwrap()
            .set_default("db_connect_retry_delay_secs", 2)
            .unwrap()
            .set_default("db_acquire_timeout_secs", 5)
            .unwrap()
            .set_default("db_max_connections", 5)
            .unwrap()
            .set_default("cors_allowed_origins", vec!["http://localhost:5173"])
            .unwrap()
            .set_default("cors_allowed_methods", vec!["GET", "POST", "OPTIONS"])
            .unwrap()
            .set_default(
                "postgres_url",
                "postgres://rewardio:rewardio@localhost:5432/rewardio",
            )
            .unwrap()
            .set_override(
                "cors_allowed_origins",
                vec!["https://example.com", "https://admin.example.com"],
            )
            .unwrap()
            .add_source(
                config::Environment::with_prefix("REWARDIO_TEST")
                    .separator("__")
                    .list_separator(",")
                    .with_list_parse_key("cors_allowed_origins")
                    .with_list_parse_key("cors_allowed_methods")
                    .with_list_parse_key("CORS_ALLOWED_ORIGINS")
                    .with_list_parse_key("CORS_ALLOWED_METHODS"),
            );

        let config: Config = builder.build().unwrap().try_deserialize().unwrap();

        let server_port = config.server_port;
        let log_level = config.log_level.clone();
        let db_connect_max_retries = config.db_connect_max_retries;
        let cors_allowed_origins = config.cors_allowed_origins.clone();

        unsafe {
            env::remove_var("REWARDIO_TEST__SERVER_PORT");
            env::remove_var("REWARDIO_TEST__LOG_LEVEL");
            env::remove_var("REWARDIO_TEST__DB_CONNECT_MAX_RETRIES");
            env::remove_var("REWARDIO_TEST__CORS_ALLOWED_ORIGINS__0");
            env::remove_var("REWARDIO_TEST__CORS_ALLOWED_ORIGINS__1");
        }

        assert_eq!(server_port, 4000);
        assert_eq!(log_level, "debug");
        assert_eq!(db_connect_max_retries, 7);
        assert_eq!(
            cors_allowed_origins,
            vec!["https://example.com", "https://admin.example.com"]
        );
    }

    #[test]
    fn test_validate_rejects_wildcard_origin_in_production() {
        let config = Config {
            app_env: "production".to_string(),
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
            cors_allowed_origins: vec!["*".to_string()],
            cors_allowed_methods: vec!["GET".to_string()],
        };

        assert!(config.validate().is_err());
    }
}
