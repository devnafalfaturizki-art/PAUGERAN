//! Structured logging utilities built on top of `tracing`.

pub use tracing::{debug, error, info, instrument, trace, warn};

pub fn init() {
    use tracing_subscriber::{fmt, EnvFilter};
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,paugeran=debug"));
    let _ = fmt().with_env_filter(filter).try_init();
}

pub fn init_for_test() {
    use tracing_subscriber::{fmt, EnvFilter};
    let _ = fmt()
        .with_env_filter(EnvFilter::new("warn"))
        .with_test_writer()
        .try_init();
}