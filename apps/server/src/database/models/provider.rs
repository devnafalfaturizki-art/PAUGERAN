use serde::{Deserialize, Serialize};

/// API-facing representation of an LLM provider configuration.
/// The `api_key` field carries the decrypted key for the current user only;
/// the database stores it encrypted at rest.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Provider {
    pub id: String,
    pub name: String,
    pub provider_type: String,
    pub api_key: String,
    pub model: String,
    pub created_at: String,
}

/// Internal persistence record (lives behind the repository).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderRecord {
    pub id: String,
    pub name: String,
    pub provider_type: String,
    pub api_key: String,
    pub model: String,
    pub created_at: String,
}

/// Payload accepted by the provider endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewProvider {
    pub name: String,
    pub provider_type: String,
    pub api_key: String,
    pub model: String,
}

impl NewProvider {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("name wajib diisi".into());
        }
        if self.provider_type.trim().is_empty() {
            return Err("provider_type wajib diisi".into());
        }
        if self.api_key.trim().is_empty() {
            return Err("api_key wajib diisi".into());
        }
        if self.model.trim().is_empty() {
            return Err("model wajib diisi".into());
        }
        Ok(())
    }
}