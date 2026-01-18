use crate::config::Config;
use std::sync::Once;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{EnvFilter, Layer, fmt, layer::SubscriberExt, util::SubscriberInitExt};

static INIT: Once = Once::new();

pub struct LogGuards {
    pub guard: Option<WorkerGuard>,
}

pub fn init_logger(config: &Config) -> LogGuards {
    let mut guard = None;
    INIT.call_once(|| {
        let env_filter =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(&config.log_level));

        let mut layers = Vec::new();

        if config.log_to_console {
            let console_layer = fmt::layer()
                .with_ansi(true)
                .with_level(true)
                .with_target(true)
                .with_line_number(true)
                .with_file(true)
                .with_thread_ids(false)
                .with_thread_names(false)
                .boxed();
            layers.push(console_layer);
        }

        if config.log_to_file {
            let file_appender = tracing_appender::rolling::daily(&config.log_dir, "app.log");
            let (non_blocking, g) = tracing_appender::non_blocking(file_appender);

            guard = Some(g);

            let file_layer = fmt::layer()
                .with_writer(non_blocking)
                .with_ansi(false)
                .fmt_fields(fmt::format::debug_fn(|writer, field, value| {
                    write!(writer, "{}={:?}", field, value)
                }))
                .with_level(true)
                .with_target(true)
                .with_line_number(true)
                .with_file(true)
                .with_thread_ids(false)
                .with_thread_names(false)
                .boxed();
            layers.push(file_layer);
        }

        let _ = tracing_subscriber::registry()
            .with(env_filter)
            .with(layers)
            .try_init();
    });

    LogGuards { guard }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[test]
    fn test_init_logger_no_panic() {
        let config = Config {
            server_port: 3000,
            log_level: "info".to_string(),
            log_to_file: false,
            log_to_console: true,
            log_dir: "logs/back_logs".to_string(),
            database_path: "users.json".to_string(),
        };

        let _guards = init_logger(&config);
        let _guards = init_logger(&config);
    }
}
