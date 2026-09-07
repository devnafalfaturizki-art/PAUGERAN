//! Application configuration structure.
//! 
//! [CB §4.6] — Configuration module

use std::path::PathBuf;

use super::{data_dir::default_data_dir, defaults::*, env_loader::load_config};

/// Main application configuration.
/// 
/// [CB §57] — Single Binary Deployment
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// Server host address
    pub host: String,
    /// Server port
    pub port: u16,
    /// Data directory path
    pub data_dir: PathBuf,
    /// Optional full database URL override
    pub database_url: Option<String>,
}

impl AppConfig {
    /// Load configuration from environment variables with defaults.
    pub fn from_env() -> Self {
        let (host, port, data_dir, database_url) = load_config();
        
        let data_dir = if data_dir == PathBuf::from(DEFAULT_DATA_DIR) {
            default_data_dir()
        } else {
            data_dir
        };

        Self {
            host,
            port,
            data_dir,
            database_url,
        }
    }

    /// Get the full socket address.
    pub fn address(&self) -> Result<std::net::SocketAddr, std::net::AddrParseError> {
        format!("{}:{}", self.host, self.port).parse()
    }

    /// Get the database path.
    pub fn db_path(&self) -> PathBuf {
        self.data_dir.join("paugeran.db")
    }

    /// Get the secrets file path.
    pub fn secrets_path(&self) -> PathBuf {
        self.data_dir.join(".secret")
    }

    /// Get the logs directory path.
    pub fn logs_dir(&self) -> PathBuf {
        self.data_dir.join("logs")
    }

    /// Get the documents directory path.
    pub fn documents_dir(&self) -> PathBuf {
        self.data_dir.join("documents")
    }

    /// Get the exports directory path.
    pub fn exports_dir(&self) -> PathBuf {
        self.data_dir.join("exports")
    }
}
