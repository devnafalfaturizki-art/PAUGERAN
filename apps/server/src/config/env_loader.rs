//! Environment variable loader for PAUGERAN configuration.
//!
//! [CB §4.6] — Configuration module

use std::{env, path::PathBuf};

use super::defaults::{DEFAULT_DATA_DIR, DEFAULT_HOST, DEFAULT_PORT, ENV_PREFIX};

/// Load application configuration from environment variables.
///
/// Recognised variables:
/// - `PAUGERAN_HOST` / `HOST` — bind address
/// - `PAUGERAN_PORT` / `PORT` — TCP port
/// - `PAUGERAN_DATA_DIR` / `DATA_DIR` — persistent data directory
/// - `PAUGERAN_DATABASE_URL` / `DATABASE_URL` — full SQLite URL override
pub fn load_config() -> (String, u16, PathBuf, Option<String>) {
    let host = env::var(format!("{ENV_PREFIX}HOST"))
        .or_else(|_| env::var("HOST"))
        .unwrap_or_else(|_| DEFAULT_HOST.to_string());

    let port = env::var(format!("{ENV_PREFIX}PORT"))
        .or_else(|_| env::var("PORT"))
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(DEFAULT_PORT);

    let data_dir = env::var(format!("{ENV_PREFIX}DATA_DIR"))
        .or_else(|_| env::var("DATA_DIR"))
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(DEFAULT_DATA_DIR));

    let database_url = env::var(format!("{ENV_PREFIX}DATABASE_URL"))
        .or_else(|_| env::var("DATABASE_URL"))
        .ok();

    (host, port, data_dir, database_url)
}

/// Convenience helper used by `bin` startup code to derive a
/// [`std::net::SocketAddr`] without re-parsing the struct.
#[allow(dead_code)]
pub fn load_socket_addr() -> Result<std::net::SocketAddr, std::net::AddrParseError> {
    let (host, port, _, _) = load_config();
    format!("{host}:{port}").parse()
}