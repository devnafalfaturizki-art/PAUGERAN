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
use uuid::Uuid;

pub async fn list_documents(
    State(state): State<Arc<AppState>>,
    Path(case_id): Path<String>,
) -> Result<Json<Vec<crate::database::models::document::DocumentRecord>>, AppError> {
    let docs = state.database.list_documents(&case_id).await?;
    Ok(Json(docs))
}

pub async fn upload_document(
    State(state): State<Arc<AppState>>,
    Path(case_id): Path<String>,
) -> Result<Json<crate::database::models::document::DocumentRecord>, AppError> {
    let record = crate::database::models::document::DocumentRecord {
        id: Uuid::new_v4().to_string(),
        case_id,
        filename: "uploaded.pdf".into(),
        content_type: "application/pdf".into(),
        size: 0,
        created_at: chrono::Utc::now().to_rfc3339(),
    };
    state.database.save_document(&record).await?;
    Ok(Json(record))
}
