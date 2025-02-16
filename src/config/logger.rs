use env_logger::{Builder, Env};

pub struct LoggerConfig;

impl LoggerConfig {
    pub fn init(rust_log: &str) {
        Builder::from_env(Env::default().default_filter_or(rust_log))
            .format_timestamp_millis()
            .format_module_path(true)
            .init();
    }
}