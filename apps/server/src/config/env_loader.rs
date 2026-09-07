//! Environment variable loader for PAUGERAN configuration.
//! 
//! [CB §4.6] — Configuration module

use std::{env, net::SocketAddr, path::PathBuf};

use super::defaults::{DEFAULT_DATA_DIR, DEFAULT_HOST, DEFAULT_PORT, ENV_PREFIX};

/// Load application configuration from environment variables.
/// 
/// Environment variables:
/// - `PAUGERAN_HOST` — Server host (default: 127.0.0.1)
/// - `PAUGERAN_PORT` — Server port (default: 3000)
/// - `PAUGERAN_DATA_DIR` — Data directory path (default: ./data)
/// - `HOST` — Alternative host variable
/// - `PORT` — Alternative port variable
/// - `DATA_DIR` — Alternative data dir variable
pub fn load_config() -> (String, u16, PathBuf) {
    let host = env::var(format!("{}HOST", ENV_PREFIX))
        .or_else(|_| env::var("HOST"))
        .unwrap_or_else(|_| DEFAULT_HOST.to_string());

    let port = env::var(format!("{}PORT", ENV_PREFIX))
        .or_else(|_| env::var("PORT"))
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_PORT);

    let data_dir = env::var(format!("{}DATA_DIR", ENV_PREFIX))
        .or_else(|_| env::var("DATA_DIR"))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(DEFAULT_DATA_DIR));

    (host, port, data_dir)
}

#[allow(dead_code)]
pub fn load_socket_addr() -> Result<SocketAddr, std::net::AddrParseError> {
    let (host, port, _) = load_config();
    format!("{}:{}", host, port).parse()
}
