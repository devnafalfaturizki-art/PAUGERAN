//! Encryption key management boundary.
//!
//! [CB §62] — Encryption & Privacy

use std::path::PathBuf;

use crate::crypto::aes_gcm::{decrypt, encrypt};

const SECRET_FILE: &str = ".secret";
const KEY_SIZE: usize = 32;

/// Resolve the path to the `.secret` file inside the data directory.
pub fn secrets_path(data_dir: &std::path::Path) -> PathBuf {
    data_dir.join(SECRET_FILE)
}

/// Load an existing key or create a new random 256-bit key on disk.
///
/// The file is created with permission `0o600` on Unix platforms.
pub fn load_or_create_key(data_dir: &std::path::Path) -> anyhow::Result<[u8; KEY_SIZE]> {
    let path = secrets_path(data_dir);

    if path.exists() {
        let key_bytes = std::fs::read(&path)?;
        if key_bytes.len() == KEY_SIZE {
            let mut key = [0u8; KEY_SIZE];
            key.copy_from_slice(&key_bytes);
            let _ = set_permission_600(&path);
            return Ok(key);
        }
        anyhow::bail!("secret file has invalid length: {}", key_bytes.len());
    }

    std::fs::create_dir_all(data_dir)?;
    let mut key = [0u8; KEY_SIZE];
    rand::random::<[u8; KEY_SIZE]>().clone_into(&mut key);
    std::fs::write(&path, &key)?;
    set_permission_600(&path)?;
    Ok(key)
}

#[cfg(unix)]
fn set_permission_600(path: &std::path::Path) -> anyhow::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600))?;
    Ok(())
}

#[cfg(not(unix))]
fn set_permission_600(_path: &std::path::Path) -> anyhow::Result<()> {
    Ok(())
}

/// Encrypt an API key using the data-directory master key.
pub fn encrypt_api_key(plaintext: &str, data_dir: &std::path::Path) -> anyhow::Result<String> {
    let key = load_or_create_key(data_dir)?;
    encrypt(&key, plaintext)
}

/// Decrypt an API key using the data-directory master key.
pub fn decrypt_api_key(ciphertext: &str, data_dir: &std::path::Path) -> anyhow::Result<String> {
    let key = load_or_create_key(data_dir)?;
    decrypt(&key, ciphertext)
}
