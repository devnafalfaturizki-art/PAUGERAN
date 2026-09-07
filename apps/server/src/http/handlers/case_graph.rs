use crate::{
    case_graph::{CaseEdge, CaseNode},
    database::{CaseEdgeRecord, CaseNodeRecord},
    error::AppError,
    http::{
        handlers::types::{
            CreateEdgeRequest, CreateNodeRequest, GraphEdgeResponse, GraphNodeResponse,
            GraphResponse,
        },
        state::AppState,
    },
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use std::sync::Arc;
use uuid::Uuid;

async fn ensure_case(state: &AppState, id: &str) -> Result<(), AppError> {
    state
        .database
        .find_case(id)
        .await?
        .ok_or_else(|| AppError::NotFound("perkara tidak ditemukan".into()))
        .map(|_| ())
}
fn node_response(record: CaseNodeRecord) -> GraphNodeResponse {
    GraphNodeResponse {
        id: record.id,
        case_id: record.case_id,
        node_type: record.node_type,
        content: record.content,
        metadata: serde_json::from_str(&record.metadata).unwrap_or_else(|_| serde_json::json!({})),
        created_at: record.created_at,
    }
}
fn edge_response(record: CaseEdgeRecord) -> GraphEdgeResponse {
    GraphEdgeResponse {
        id: record.id,
        case_id: record.case_id,
        source_node_id: record.source_node_id,
        target_node_id: record.target_node_id,
        edge_type: record.edge_type,
        metadata: serde_json::from_str(&record.metadata).unwrap_or_else(|_| serde_json::json!({})),
        created_at: record.created_at,
    }
}

pub async fn case_graph(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<GraphResponse>, AppError> {
    ensure_case(&state, &id).await?;
    Ok(Json(GraphResponse {
        nodes: state
            .database
            .list_nodes(&id)
            .await?
            .into_iter()
            .map(node_response)
            .collect(),
        edges: state
            .database
            .list_edges(&id)
            .await?
            .into_iter()
            .map(edge_response)
            .collect(),
    }))
}

pub async fn create_graph_node(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(payload): Json<CreateNodeRequest>,
) -> Result<(StatusCode, Json<CaseNode>), AppError> {
    ensure_case(&state, &id).await?;
    if payload.content.trim().is_empty() {
        return Err(AppError::Validation("content wajib diisi".into()));
    }
    let node = CaseNode {
        id: Uuid::new_v4().to_string(),
        case_id: id,
        node_type: payload.node_type,
        content: payload.content.trim().into(),
        metadata: payload.metadata,
    };
    state.database.create_node(&node).await?;
    Ok((StatusCode::CREATED, Json(node)))
}

pub async fn create_graph_edge(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(payload): Json<CreateEdgeRequest>,
) -> Result<StatusCode, AppError> {
    ensure_case(&state, &id).await?;
    let nodes = state.database.list_nodes(&id).await?;
    if !nodes.iter().any(|node| node.id == payload.source_node_id)
        || !nodes.iter().any(|node| node.id == payload.target_node_id)
    {
        return Err(AppError::Validation(
            "node harus berada dalam perkara yang sama".into(),
        ));
    }
    let edge = CaseEdge {
        id: Uuid::new_v4().to_string(),
        case_id: id,
        source_node_id: payload.source_node_id,
        target_node_id: payload.target_node_id,
        edge_type: payload.edge_type,
        metadata: payload.metadata,
    };
    state.database.create_edge(&edge).await?;
    Ok(StatusCode::CREATED)
}
