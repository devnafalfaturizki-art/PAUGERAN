//! AES-256-GCM authenticated encryption boundary.
//!
//! [CB §62] — Encryption & Privacy

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};

/// Encrypt plaintext with AES-256-GCM.
///
/// Returns base64( nonce || ciphertext ).
pub fn encrypt(key: &[u8; 32], plaintext: &str) -> anyhow::Result<String> {
    let cipher = Aes256Gcm::new(key.into());
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher
        .encrypt(&nonce, plaintext.as_bytes())
        .map_err(|e| anyhow::anyhow!("aes-gcm encrypt failed: {e}"))?;
    let mut result = nonce.to_vec();
    result.extend_from_slice(&ciphertext);
    Ok(BASE64.encode(result))
}

/// Decrypt base64( nonce || ciphertext ) with AES-256-GCM.
pub fn decrypt(key: &[u8; 32], ciphertext: &str) -> anyhow::Result<String> {
    let data = BASE64.decode(ciphertext).map_err(|e| anyhow::anyhow!("base64 decode failed: {e}"))?;
    if data.len() < 12 {
        anyhow::bail!("ciphertext too short");
    }
    let nonce: [u8; 12] = data[..12]
        .try_into()
        .map_err(|_| anyhow::anyhow!("invalid nonce length"))?;
    let cipher = Aes256Gcm::new(key.into());
    let plaintext = cipher
        .decrypt(&nonce.into(), &data[12..])
        .map_err(|e| anyhow::anyhow!("aes-gcm decrypt failed: {e}"))?;
    String::from_utf8(plaintext).map_err(|e| anyhow::anyhow!("invalid utf8: {e}"))
}
