use crate::{
    database::Database,
    error::AppError,
    http::state::AppState,
};
use axum::{
    extract::State,
    response::Json,
};
use std::sync::Arc;

pub async fn get_preferences(
    State(state): State<Arc<AppState>>,
) -> Result<Json<crate::database::models::preference::PreferenceRecord>, AppError> {
    let pref = state.database.get_preference("global").await?;
    Ok(Json(pref.unwrap_or_default()))
}

pub async fn save_preferences(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<crate::database::models::preference::PreferenceRecord>,
) -> Result<Json<crate::database::models::preference::PreferenceRecord>, AppError> {
    state.database.save_preference(&payload).await?;
    Ok(Json(payload))
}
