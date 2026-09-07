//! Application configuration loaded from environment variables.
//! 
//! [CB §4.6] — Configuration module

pub mod app_config;
pub mod defaults;
pub mod data_dir;
pub mod env_loader;

pub use app_config::AppConfig;
pub use defaults::*;
