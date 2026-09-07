//! Global provider management handlers.
/// [CB §27] — Admin global LLM providers

use axum::{
    extract::State,
    Json,
};
use std::sync::Arc;

use crate::http::state::AppState;

/// List global LLM providers.
pub async fn list_global_providers(
    State(_state): State<Arc<AppState>>,
) -> Result<Json<Vec<crate::database::models::Provider>>, crate::error::AppError> {
    Ok(Json(vec![]))
}

/// Save global LLM provider configuration.
pub async fn save_global_provider(
    State(_state): State<Arc<AppState>>,
    Json(_payload): Json<crate::database::models::Provider>,
) -> Result<Json<crate::database::models::Provider>, crate::error::AppError> {
    Ok(Json(_payload))
}
