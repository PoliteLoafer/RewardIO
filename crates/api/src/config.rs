use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server_port: u16,
    pub log_level: String,
    pub log_to_file: bool,
    pub log_to_console: bool,
    pub log_dir: String,
    pub database_path: String,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        dotenvy::dotenv().ok();

        let builder = config::Config::builder()
            .set_default("server_port", 3000)?
            .set_default("log_level", "info")?
            .set_default("log_to_file", false)?
            .set_default("log_to_console", true)?
            .set_default("log_dir", "logs/back_logs")?
            .set_default("database_path", "users.json")?
            .add_source(config::Environment::default())
            .add_source(config::Environment::with_prefix("REWARDIO").separator("__"));

        builder.build()?.try_deserialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_config_defaults() {
        unsafe {
            env::remove_var("SERVER_PORT");
            env::remove_var("LOG_LEVEL");
            env::remove_var("REWARDIO__SERVER_PORT");
            env::remove_var("REWARDIO__LOG_LEVEL");
        }

        let builder = config::Config::builder()
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
            .set_default("database_path", "users.json")
            .unwrap();

        let config: Config = builder.build().unwrap().try_deserialize().unwrap();
        assert_eq!(config.server_port, 3000);
        assert_eq!(config.log_level, "info");
    }

    #[test]
    fn test_config_override() {
        unsafe {
            env::set_var("REWARDIO_TEST__SERVER_PORT", "4000");
            env::set_var("REWARDIO_TEST__LOG_LEVEL", "debug");
        }

        let builder = config::Config::builder()
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
            .set_default("database_path", "users.json")
            .unwrap()
            .add_source(config::Environment::with_prefix("REWARDIO_TEST").separator("__"));

        let config: Config = builder.build().unwrap().try_deserialize().unwrap();

        let server_port = config.server_port;
        let log_level = config.log_level.clone();

        unsafe {
            env::remove_var("REWARDIO_TEST__SERVER_PORT");
            env::remove_var("REWARDIO_TEST__LOG_LEVEL");
        }

        assert_eq!(server_port, 4000);
        assert_eq!(log_level, "debug");
    }
}
