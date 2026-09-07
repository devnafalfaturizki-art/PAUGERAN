//! Platform-specific data directory resolution.
//! 
//! [CB §4.6] — Configuration module

use std::path::PathBuf;

/// Resolve the default data directory based on the current platform.
/// 
/// [CB §57] — Data directory specification
pub fn default_data_dir() -> PathBuf {
    let base = dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."));
    base.join("paugeran")
}

/// Ensure the data directory exists, creating it if necessary.
/// 
/// [CB §62] — Data storage security
pub fn ensure_data_dir(path: &std::path::Path) -> std::io::Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}
