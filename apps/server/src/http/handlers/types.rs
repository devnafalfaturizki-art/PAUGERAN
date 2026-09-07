use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::case_graph::{EdgeType, NodeType};
use crate::engine::{mode_router::ReasoningMode, state_machine::CaseState};

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub service: &'static str,
    pub started_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CaseSummary {
    pub id: String,
    pub title: String,
    pub state: String,
    pub mode: String,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCaseRequest {
    pub title: String,
}
#[derive(Debug, Deserialize)]
pub struct StateUpdateRequest {
    pub state: CaseState,
    pub reason: String,
}
#[derive(Debug, Deserialize)]
pub struct ModeUpdateRequest {
    pub mode: ReasoningMode,
}
#[derive(Debug, Deserialize)]
pub struct MessageRequest {
    pub content: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateNodeRequest {
    pub node_type: NodeType,
    pub content: String,
    #[serde(default)]
    pub metadata: serde_json::Value,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEdgeRequest {
    pub source_node_id: String,
    pub target_node_id: String,
    pub edge_type: EdgeType,
    #[serde(default)]
    pub metadata: serde_json::Value,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalysisResponse {
    pub role: &'static str,
    pub content: String,
    pub mode: String,
    pub certainty_score: f32,
    pub factors: Vec<String>,
    pub clarifying_questions: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphResponse {
    pub nodes: Vec<GraphNodeResponse>,
    pub edges: Vec<GraphEdgeResponse>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphNodeResponse {
    pub id: String,
    pub case_id: String,
    pub node_type: String,
    pub content: String,
    pub metadata: serde_json::Value,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphEdgeResponse {
    pub id: String,
    pub case_id: String,
    pub source_node_id: String,
    pub target_node_id: String,
    pub edge_type: String,
    pub metadata: serde_json::Value,
    pub created_at: String,
}
