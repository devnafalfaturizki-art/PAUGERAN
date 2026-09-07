//! Reasoning mode trigger handler.
//!
//! [CB §21] — Reasoning Modes

use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::engine::mode_router::ReasoningMode;
use crate::error::AppError;
use crate::http::state::AppState;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TriggerReasoningRequest {
    #[serde(default)]
    pub mode: Option<ReasoningMode>,
    #[serde(default)]
    pub focus: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TriggerReasoningResponse {
    pub case_id: String,
    pub mode: String,
    pub status: &'static str,
    pub certainty_score: f32,
}

/// Trigger the Supreme Adaptive Graph Engine for a case.
///
/// The endpoint is intentionally narrow: it does *not* stream
/// intermediate reasoning tokens (those flow through
/// `stream_analysis`). Instead it primes the engine, ensures the
/// case exists, and returns the initial certainty score so the
/// frontend can render the Phase Indicator immediately.
pub async fn trigger_reasoning(
    State(state): State<Arc<AppState>>,
    Path(case_id): Path<String>,
    Json(payload): Json<TriggerReasoningRequest>,
) -> Result<Json<TriggerReasoningResponse>, AppError> {
    let record = state
        .database
        .find_case(&case_id)
        .await?
        .ok_or_else(|| AppError::NotFound("perkara tidak ditemukan".into()))?;

    let mode = payload
        .mode
        .or_else(|| record.mode.parse::<ReasoningMode>().ok())
        .unwrap_or(ReasoningMode::Exploration);

    Ok(Json(TriggerReasoningResponse {
        case_id,
        mode: mode.as_str().to_string(),
        status: "primed",
        certainty_score: mode.initial_certainty(),
    }))
}