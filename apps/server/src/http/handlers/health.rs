use crate::http::{handlers::types::HealthResponse, state::AppState};
use axum::{extract::State, response::Json};
use std::sync::Arc;

pub async fn health(State(state): State<Arc<AppState>>) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "paugeran",
        started_at: state.started_at,
    })
}
