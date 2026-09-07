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
    response::{
        sse::{Event, Sse},
        Json,
    },
};
use std::{convert::Infallible, sync::Arc};
use tokio_stream::iter;

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
    let redacted_content = state.pii_redactor.redact(payload.content.trim());
    state
        .database
        .save_message(&id, "user", &redacted_content, None)
        .await?;
    let certainty_score = mode.initial_certainty();
    let response = AnalysisResponse { role: "system", content: format!("Analisis awal mode {} diterima. Fakta utama perlu dipetakan sebelum kesimpulan hukum diberikan.", mode.as_str()), mode: mode.as_str().into(), certainty_score, factors: vec!["Belum ada dokumen atau sumber hukum yang diverifikasi.".into(), "Kronologi dan posisi para pihak masih perlu dilengkapi.".into()], clarifying_questions: mode.opening_questions().iter().map(|question| (*question).to_string()).collect() };
    state
        .database
        .save_message(&id, response.role, &response.content, Some(certainty_score))
        .await?;
    Ok(Json(response))
}

pub async fn stream_analysis(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Sse<impl tokio_stream::Stream<Item = Result<Event, Infallible>>>, AppError> {
    let record = state
        .database
        .find_case(&id)
        .await?
        .ok_or_else(|| AppError::NotFound("perkara tidak ditemukan".into()))?;
    let mode = record
        .mode
        .parse::<ReasoningMode>()
        .map_err(|_| AppError::Internal("mode tidak dikenal".into()))?;
    let events = vec![
        Ok(Event::default()
            .event("phase")
            .data(serde_json::json!({ "phase": "facts" }).to_string())),
        Ok(Event::default().event("phase").data(
            serde_json::json!({ "phase": "interpretation", "mode": mode.as_str() }).to_string(),
        )),
        Ok(Event::default()
            .event("complete")
            .data(serde_json::json!({ "certaintyScore": mode.initial_certainty() }).to_string())),
    ];
    Ok(Sse::new(iter(events)))
}
