//! API route definitions for PAUGERAN.
//! 
//! [CB §4.7] — HTTP Server Layer

use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use std::sync::Arc;

use crate::http::{handlers, state::AppState};

/// Build the API router with all endpoints.
/// 
/// [CB §39] — API Specification
pub fn api_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(handlers::health::health))
        .route("/api/cases", get(handlers::cases::cases))
        .route("/api/cases", post(handlers::cases::create_case))
        .route("/api/cases/:id/state", patch(handlers::cases::update_case_state))
        .route("/api/cases/:id/mode", patch(handlers::cases::update_case_mode))
        .route("/api/cases/:id/messages", post(handlers::messages::analyze_message))
        .route(
            "/api/cases/:id/messages/stream",
            get(handlers::messages::stream_analysis),
        )
        .route("/api/cases/:id/graph", get(handlers::case_graph::case_graph))
        .route(
            "/api/cases/:id/graph/nodes",
            post(handlers::case_graph::create_graph_node),
        )
        .route(
            "/api/cases/:id/graph/edges",
            post(handlers::case_graph::create_graph_edge),
        )
        .route("/api/cases/:id/documents", get(handlers::documents::list_documents))
        .route(
            "/api/cases/:id/documents/upload",
            post(handlers::documents::upload_document),
        )
        .route("/api/cases/:id/export", post(handlers::export_handler::export_case))
        .route("/api/cases/:id/reasoning", post(handlers::reasoning::trigger_reasoning))
        .route("/api/knowledge", get(handlers::knowledge::list_knowledge))
        .route("/api/knowledge", post(handlers::knowledge::add_knowledge))
        .route("/api/providers", get(handlers::providers::list_providers))
        .route("/api/providers", post(handlers::providers::save_provider))
        .route("/api/preferences", get(handlers::preferences::get_preferences))
        .route("/api/preferences", post(handlers::preferences::save_preferences))
        .route("/api/setup", post(handlers::setup::setup))
        .route("/api/admin/users", get(handlers::admin::list_users))
        .route("/api/admin/users", post(handlers::admin::create_user))
        .route("/api/admin/users/:id", delete(handlers::admin::delete_user))
        .route("/api/admin/invitations", post(handlers::admin::create_invitation))
        .route("/api/admin/providers", get(handlers::admin::list_global_providers))
        .route("/api/admin/providers", post(handlers::admin::save_global_provider))
        .route("/api/admin/knowledge", get(handlers::admin::list_global_knowledge))
        .with_state(Arc::new(state))
}
