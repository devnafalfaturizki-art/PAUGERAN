//! Password hashing using Argon2.
//!
//! [CB §62-63] — Encryption & Security
//!
//! `PAUGERAN` only stores password hashes; never plain-text
//! passwords. Hashes are produced by Argon2id with a per-process
//! pepper layered on top of the salt.

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

#[derive(Debug, thiserror::Error)]
pub enum PasswordError {
    #[error("argon2 error: {0}")]
    Argon2(String),
    #[error("invalid hash format: {0}")]
    Invalid(String),
}

/// Hash a password using Argon2id with a random salt.
pub fn hash_password(password: &str) -> Result<String, PasswordError> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|error| PasswordError::Argon2(error.to_string()))
}

/// Verify a password against an existing Argon2id PHC-formatted hash.
pub fn verify_password(password: &str, hash: &str) -> Result<bool, PasswordError> {
    let parsed = PasswordHash::new(hash).map_err(|error| PasswordError::Invalid(error.to_string()))?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok())
}