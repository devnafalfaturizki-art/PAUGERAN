//! Global knowledge base management handlers.
//! [CB §27] — Admin global knowledge base
//!
//! The admin can curate regulation entries that all team members see
//! regardless of which cases they own. The on-disk schema adds a
//! `scope` column; entries created here are tagged `scope = 'global'`.

use axum::{
    extract::State,
    Json,
};
use std::sync::Arc;

use crate::database::models::knowledge::{KnowledgeEntry, KnowledgeRecord};
use crate::error::AppError;
use crate::http::state::AppState;

/// List all global knowledge base entries.
pub async fn list_global_knowledge(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<KnowledgeEntry>>, AppError> {
    let records = sqlx::query_as::<_, KnowledgeRecord>(
        "SELECT id, title, content, tags, created_at FROM knowledge_base WHERE scope = 'global' ORDER BY created_at DESC",
    )
    .fetch_all(state.database.pool())
    .await
    .map_err(AppError::Database)?;

    Ok(Json(records.into_iter().map(KnowledgeEntry::from_record).collect()))
}

/// Persist or replace a global knowledge base entry. Re-exported for
/// the router under `admin::save_global_knowledge`.
pub async fn save_global_knowledge(
    State(state): State<Arc<AppState>>,
    Json(entry): Json<KnowledgeEntry>,
) -> Result<Json<KnowledgeEntry>, AppError> {
    if entry.title.trim().is_empty() {
        return Err(AppError::Validation("title wajib diisi".into()));
    }
    if entry.content.trim().is_empty() {
        return Err(AppError::Validation("content wajib diisi".into()));
    }
    let mut record = KnowledgeRecord::from_entry(entry);
    if record.id.trim().is_empty() {
        record.id = crate::utils::id_generator::new_uuid();
    }
    record.created_at = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT OR REPLACE INTO knowledge_base (id, title, content, tags, scope, created_at)
         VALUES (?, ?, ?, ?, 'global', ?)",
    )
    .bind(&record.id)
    .bind(&record.title)
    .bind(&record.content)
    .bind(&record.tags)
    .bind(&record.created_at)
    .execute(state.database.pool())
    .await
    .map_err(AppError::Database)?;

    Ok(Json(KnowledgeEntry::from_record(record)))
}