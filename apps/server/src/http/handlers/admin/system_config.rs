//! System configuration handlers.
/// [CB §27] — Admin system configuration

use axum::{
    extract::State,
    Json,
};
use std::sync::Arc;

use crate::http::state::AppState;

/// Get system configuration.
pub async fn get_system_config(
    State(_state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, crate::error::AppError> {
    Ok(Json(serde_json::json!({"message": "Not implemented yet"})))
}

/// Update system configuration.
pub async fn update_system_config(
    State(_state): State<Arc<AppState>>,
    Json(_payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, crate::error::AppError> {
    Ok(Json(serde_json::json!({"message": "Not implemented yet"})))
}
