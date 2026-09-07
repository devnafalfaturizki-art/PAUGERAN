use crate::{
    crypto::pii_redactor::PiiRedactor,
    error::AppError,
    http::state::AppState,
};
use axum::{
    extract::State,
    response::Json,
    Json as Json2,
};
use std::sync::Arc;

#[derive(serde::Deserialize)]
pub struct SetupRequest {
    pub api_key: String,
    pub provider: String,
    pub model: String,
}

pub async fn setup(
    State(_state): State<Arc<AppState>>,
    Json2(payload): Json<SetupRequest>,
) -> Result<Json2<serde_json::Value>, AppError> {
    if payload.api_key.trim().is_empty() {
        return Err(AppError::Validation("API key wajib diisi".into()));
    }
    let _redacted = _state.pii_redactor.redact(&payload.api_key);
    Ok(Json2(serde_json::json!({
        "status": "ok",
        "provider": payload.provider,
        "model": payload.model,
        "message": "Konfigurasi berhasil disimpan"
    })))
}
