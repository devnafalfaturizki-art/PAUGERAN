//! Global provider management handlers.
//! [CB §27] — Admin global LLM providers
//!
//! Endpoints scoped to admin tokens. In single-user mode these
//! simply proxy the user-scoped provider table; in multi-user mode
//! the rows are marked `scope = 'global'` so that downstream
//! requests can fall back to them when a user has not configured
//! their own key (see [CB §29] — Fallback API key hierarchy).

use axum::{
    extract::State,
    Json,
};
use std::sync::Arc;

use crate::crypto::key_manager::{decrypt_api_key, encrypt_api_key};
use crate::database::models::provider::{Provider, ProviderRecord};
use crate::error::AppError;
use crate::http::state::AppState;
use crate::utils::id_generator::new_uuid;

async fn fetch_all_global(state: &AppState) -> Result<Vec<ProviderRecord>, AppError> {
    sqlx::query_as::<_, (String, String, String, String, String, String)>(
        "SELECT id, name, provider_type, api_key, model, created_at FROM providers WHERE scope = 'global' ORDER BY created_at DESC",
    )
    .fetch_all(state.database.pool())
    .await
    .map(|rows| {
        rows.into_iter()
            .map(|(id, name, provider_type, encrypted_api_key, model, created_at)| ProviderRecord {
                id,
                name,
                provider_type,
                api_key: decrypt_api_key(&encrypted_api_key, state.database.data_dir())
                    .unwrap_or_default(),
                model,
                created_at,
            })
            .collect()
    })
    .map_err(AppError::Database)
}

/// List global LLM providers configured by the admin.
pub async fn list_global_providers(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Provider>>, AppError> {
    let records = fetch_all_global(&state).await?;
    Ok(Json(records.into_iter().map(Into::into).collect()))
}

/// Persist or replace a global LLM provider configuration.
pub async fn save_global_provider(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Provider>,
) -> Result<Json<Provider>, AppError> {
    if payload.api_key.trim().is_empty() {
        return Err(AppError::Validation("api_key wajib diisi".into()));
    }
    let encrypted = encrypt_api_key(&payload.api_key, state.database.data_dir())
        .map_err(|error| AppError::Internal(format!("gagal enkripsi api_key: {error}")))?;

    let record = ProviderRecord {
        id: if payload.id.trim().is_empty() {
            new_uuid()
        } else {
            payload.id.clone()
        },
        name: payload.name,
        provider_type: payload.provider_type,
        api_key: payload.api_key,
        model: payload.model,
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    sqlx::query(
        "INSERT OR REPLACE INTO providers (id, name, provider_type, api_key, model, scope, created_at)
         VALUES (?, ?, ?, ?, ?, 'global', ?)",
    )
    .bind(&record.id)
    .bind(&record.name)
    .bind(&record.provider_type)
    .bind(encrypted)
    .bind(&record.model)
    .bind(&record.created_at)
    .execute(state.database.pool())
    .await
    .map_err(AppError::Database)?;

    Ok(Json(record.into()))
}

impl From<ProviderRecord> for Provider {
    fn from(value: ProviderRecord) -> Self {
        Self {
            id: value.id,
            name: value.name,
            provider_type: value.provider_type,
            api_key: value.api_key,
            model: value.model,
            created_at: value.created_at,
        }
    }
}