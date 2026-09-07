use crate::{
    database::Database,
    error::AppError,
    http::state::AppState,
};
use axum::{
    extract::{Path, State},
    response::Json,
};
use std::sync::Arc;

pub async fn list_knowledge(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<crate::database::models::knowledge::KnowledgeRecord>>, AppError> {
    let entries = state.database.list_knowledge().await?;
    Ok(Json(entries))
}

pub async fn add_knowledge(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<crate::database::models::knowledge::KnowledgeRecord>,
) -> Result<Json<crate::database::models::knowledge::KnowledgeRecord>, AppError> {
    state.database.save_knowledge(&payload).await?;
    Ok(Json(payload))
}
