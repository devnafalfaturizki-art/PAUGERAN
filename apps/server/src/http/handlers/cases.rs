use crate::{
    database::{CaseRecord, Database},
    engine::state_machine::{CaseState, CaseStateMachine},
    error::AppError,
    http::{
        handlers::types::{CaseSummary, CreateCaseRequest, ModeUpdateRequest, StateUpdateRequest},
        state::AppState,
    },
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use std::sync::Arc;

fn state_from_str(value: &str) -> Option<CaseState> {
    match value {
        "unknown" => Some(CaseState::Unknown),
        "exploration" => Some(CaseState::Exploration),
        "preventive" => Some(CaseState::Preventive),
        "dispute" => Some(CaseState::Dispute),
        "litigation" => Some(CaseState::Litigation),
        "resolved" => Some(CaseState::Resolved),
        _ => None,
    }
}

pub fn summary(record: CaseRecord) -> Result<CaseSummary, AppError> {
    let state = state_from_str(&record.state)
        .ok_or_else(|| AppError::Internal(format!("state tidak dikenal: {}", record.state)))?;
    Ok(CaseSummary {
        id: record.id,
        title: record.title,
        state: state.as_str().to_string(),
        mode: record.mode,
        updated_at: Database::parse_timestamp(&record.updated_at),
    })
}

pub async fn cases(State(state): State<Arc<AppState>>) -> Result<Json<Vec<CaseSummary>>, AppError> {
    state
        .database
        .list_cases()
        .await?
        .into_iter()
        .map(summary)
        .collect::<Result<Vec<_>, _>>()
        .map(Json)
}

pub async fn create_case(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateCaseRequest>,
) -> Result<(StatusCode, Json<CaseSummary>), AppError> {
    if payload.title.trim().is_empty() {
        return Err(AppError::Validation("title wajib diisi".into()));
    }
    let record = state
        .database
        .create_case(&payload.title, CaseState::Unknown, "exploration")
        .await?;
    Ok((StatusCode::CREATED, Json(summary(record)?)))
}

pub async fn update_case_state(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(payload): Json<StateUpdateRequest>,
) -> Result<Json<CaseSummary>, AppError> {
    let record = state
        .database
        .find_case(&id)
        .await?
        .ok_or_else(|| AppError::NotFound("perkara tidak ditemukan".into()))?;
    let current = state_from_str(&record.state)
        .ok_or_else(|| AppError::Internal("state tidak dikenal".into()))?;
    let mut machine = CaseStateMachine::new(current);
    machine
        .transition(payload.state, payload.reason)
        .map_err(|error| AppError::Validation(error.to_string()))?;
    let updated = state
        .database
        .update_state(&id, payload.state)
        .await?
        .ok_or_else(|| AppError::NotFound("perkara tidak ditemukan".into()))?;
    Ok(Json(summary(updated)?))
}

pub async fn update_case_mode(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(payload): Json<ModeUpdateRequest>,
) -> Result<Json<CaseSummary>, AppError> {
    let updated = state
        .database
        .update_mode(&id, payload.mode)
        .await?
        .ok_or_else(|| AppError::NotFound("perkara tidak ditemukan".into()))?;
    Ok(Json(summary(updated)?))
}
