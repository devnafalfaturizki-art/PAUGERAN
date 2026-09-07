//! Global knowledge base management handlers.
/// [CB §27] — Admin global knowledge base

use axum::{
    extract::State,
    Json,
};
use std::sync::Arc;

use crate::http::state::AppState;

/// List global knowledge base entries.
pub async fn list_global_knowledge(
    State(_state): State<Arc<AppState>>,
) -> Result<Json<Vec<crate::database::models::KnowledgeEntry>>, crate::error::AppError> {
    Ok(Json(vec![]))
}
