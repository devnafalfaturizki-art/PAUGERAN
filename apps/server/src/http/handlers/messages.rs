use crate::{
    engine::mode_router::ReasoningMode,
    error::AppError,
    http::{
        handlers::types::{AnalysisResponse, MessageRequest},
        state::AppState,
    },
};
use axum::{
    extract::{Path, State},
    response::Json,
};
use std::sync::Arc;

pub async fn analyze_message(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(payload): Json<MessageRequest>,
) -> Result<Json<AnalysisResponse>, AppError> {
    if payload.content.trim().is_empty() {
        return Err(AppError::Validation("content wajib diisi".into()));
    }
    let record = state
        .database
        .find_case(&id)
        .await?
        .ok_or_else(|| AppError::NotFound("perkara tidak ditemukan".into()))?;
    let mode = record
        .mode
        .parse::<ReasoningMode>()
        .map_err(|_| AppError::Internal("mode tidak dikenal".into()))?;
    state
        .database
        .save_message(&id, "user", payload.content.trim(), None)
        .await?;
    let certainty_score = mode.initial_certainty();
    let response = AnalysisResponse { role: "system", content: format!("Analisis awal mode {} diterima. Fakta utama perlu dipetakan sebelum kesimpulan hukum diberikan.", mode.as_str()), mode: mode.as_str().into(), certainty_score, factors: vec!["Belum ada dokumen atau sumber hukum yang diverifikasi.".into(), "Kronologi dan posisi para pihak masih perlu dilengkapi.".into()], clarifying_questions: mode.opening_questions().iter().map(|question| (*question).to_string()).collect() };
    state
        .database
        .save_message(&id, response.role, &response.content, Some(certainty_score))
        .await?;
    Ok(Json(response))
}
