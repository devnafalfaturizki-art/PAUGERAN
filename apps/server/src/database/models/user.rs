use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Public user representation (no password hash leaked to clients).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub email: String,
    pub role: String,
    pub created_at: String,
}

/// Internal record persisted in the database (includes password hash).
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct UserRecord {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub created_at: String,
}

/// Payload accepted by the admin endpoint when creating a user.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewUser {
    pub email: String,
    pub password: String,
    #[serde(default)]
    pub role: Option<String>,
}

impl NewUser {
    /// Validate the payload minimally before hitting persistence/crypto.
    pub fn validate(&self) -> Result<(), String> {
        if self.email.trim().is_empty() {
            return Err("email wajib diisi".into());
        }
        if self.password.len() < 8 {
            return Err("password minimal 8 karakter".into());
        }
        Ok(())
    }
}