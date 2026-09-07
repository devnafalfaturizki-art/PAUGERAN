//! JWT token issuance and validation used by the optional
//! multi-user authentication mode.

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use super::key_manager::read_or_create_secret;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}

pub struct JwtManager {
    secret: Vec<u8>,
}

impl JwtManager {
    pub fn load(data_dir: &std::path::Path) -> Result<Self, String> {
        let secret = read_or_create_secret(data_dir)?;
        Ok(Self { secret })
    }

    pub fn issue(&self, subject: &str, role: &str, ttl_seconds: u64) -> Result<String, String> {
        let exp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| e.to_string())?
            .as_secs()
            + ttl_seconds;
        let claims = Claims {
            sub: subject.to_string(),
            role: role.to_string(),
            exp: exp as usize,
        };
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(&self.secret),
        )
        .map_err(|e| e.to_string())
    }

    pub fn verify(&self, token: &str) -> Result<Claims, String> {
        let validation = Validation::default();
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(&self.secret),
            &validation,
        )
        .map(|data| data.claims)
        .map_err(|e| e.to_string())
    }
}