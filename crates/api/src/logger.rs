use crate::config::Config;
use std::backtrace::Backtrace;
use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;
use std::panic::AssertUnwindSafe;
use std::path::Path;
use std::sync::Once;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{EnvFilter, Layer, fmt, layer::SubscriberExt, util::SubscriberInitExt};

static INIT: Once = Once::new();

pub struct LogGuards {
    pub guard: Option<WorkerGuard>,
}

fn format_panic_dump(message: &str, location: &str) -> String {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let thread = thread::current();
    let thread_name = thread.name().unwrap_or("unnamed");
    let backtrace = Backtrace::force_capture();
    let backtrace_text = backtrace.to_string();
    let panic_origin_symbol = backtrace_text
        .lines()
        .map(str::trim)
        .find(|line| {
            line.contains("rewardio_api::")
                && !line.contains("logger::init_logger")
                && !line.contains("std::panicking::")
        })
        .unwrap_or("unknown")
        .to_string();

    format!(
        "===== PANIC DUMP START =====\n\ttimestamp_unix={ts}\n\tthread_name={thread_name}\n\tthread_id={:?}\n\tpanic_origin_location={location}\n\tpanic_origin_symbol={panic_origin_symbol}\n\tmessage={message}\n\tbacktrace:\n{backtrace_text}\n===== PANIC DUMP END =====\n",
        thread.id()
    )
}

fn write_panic_dump(log_dir: &str, message: &str, location: &str) {
    if create_dir_all(log_dir).is_err() {
        return;
    }

    let dump_path = format!("{log_dir}/panic.log");
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(dump_path) {
        let dump = format_panic_dump(message, location);
        let _ = file.write_all(dump.as_bytes());
        let _ = file.flush();
    }
}

fn can_write_log_file(log_dir: &str, file_name: &str) -> bool {
    if create_dir_all(log_dir).is_err() {
        return false;
    }

    let path = Path::new(log_dir).join(file_name);
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .is_ok()
}

pub fn init_logger(config: &Config) -> LogGuards {
    let mut guard = None;
    INIT.call_once(|| {
        let panic_log_dir = config.log_dir.clone();
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
            if can_write_log_file(&config.log_dir, "app.log") {
                let rolling_result = std::panic::catch_unwind(AssertUnwindSafe(|| {
                    tracing_appender::rolling::daily(&config.log_dir, "app.log")
                }));

                match rolling_result {
                    Ok(file_appender) => {
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
                    Err(_) => {
                        eprintln!(
                            "File logging disabled: failed to initialize rolling appender for {}",
                            config.log_dir
                        );
                    }
                }
            } else {
                eprintln!(
                    "File logging disabled: log directory/file is not writable: {}",
                    config.log_dir
                );
            }
        }

        let _ = tracing_subscriber::registry()
            .with(env_filter)
            .with(layers)
            .try_init();

        let default_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            let message = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
                s.clone()
            } else {
                "Unknown panic payload".to_string()
            };

            let location = panic_info
                .location()
                .map(|loc| format!("{}:{}:{}", loc.file(), loc.line(), loc.column()))
                .unwrap_or_else(|| "unknown location".to_string());

            tracing::error!(
                target: "panic",
                panic_message = %message,
                panic_location = %location,
                "A critical panic occurred"
            );

            write_panic_dump(&panic_log_dir, &message, &location);
            default_hook(panic_info);
        }));
    });

    LogGuards { guard }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use std::path::Path;

    #[test]
    fn test_init_logger_no_panic() {
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

        let _guards = init_logger(&config);
        let _guards = init_logger(&config);
    }

    #[test]
    fn test_write_panic_dump_creates_file() {
        let test_dir = "target/tmp/panic_test_logs";
        write_panic_dump(test_dir, "test panic", "src/main.rs:1:1");

        let path = format!("{test_dir}/panic.log");
        assert!(Path::new(&path).exists());
    }

    #[test]
    fn test_can_write_log_file_success() {
        let test_dir = "target/tmp/logger_write_check";
        assert!(can_write_log_file(test_dir, "app.log"));
    }

    #[test]
    fn test_format_panic_dump_contains_origin_fields() {
        let dump = format_panic_dump("test panic", "crates/api/src/main.rs:26:5");
        assert!(dump.contains("panic_origin_location=crates/api/src/main.rs:26:5"));
        assert!(dump.contains("panic_origin_symbol="));
    }
}
