use axum::{
    routing::{get, patch, post},
    Router,
};
use std::sync::Arc;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::{
    frontend::assets,
    http::{handlers, state::AppState},
};

pub fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(handlers::health))
        .route("/api/cases", get(handlers::cases))
        .route("/api/cases", post(handlers::create_case))
        .route("/api/cases/:id/state", patch(handlers::update_case_state))
        .route("/api/cases/:id/mode", patch(handlers::update_case_mode))
        .route("/api/cases/:id/messages", post(handlers::analyze_message))
        .route("/api/cases/:id/graph", get(handlers::case_graph))
        .route(
            "/api/cases/:id/graph/nodes",
            post(handlers::create_graph_node),
        )
        .route(
            "/api/cases/:id/graph/edges",
            post(handlers::create_graph_edge),
        )
        .route("/*path", get(assets::serve))
        .with_state(Arc::new(state))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}
