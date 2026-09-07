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

pub async fn list_providers(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<crate::database::models::provider::ProviderRecord>>, AppError> {
    let providers = state.database.list_providers().await?;
    Ok(Json(providers))
}

pub async fn save_provider(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<crate::database::models::provider::ProviderRecord>,
) -> Result<Json<crate::database::models::provider::ProviderRecord>, AppError> {
    state.database.save_provider(&payload).await?;
    Ok(Json(payload))
}
