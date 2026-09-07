//! Case Graph persistence adapter. Bridges the in-memory `petgraph`
//! representation with the SQLite-backed repositories.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{CaseEdge, CaseNode};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersistedNode {
    pub id: String,
    pub case_id: String,
    pub node_type: String,
    pub content: String,
    pub metadata: Value,
    pub created_at: String,
}

impl From<&CaseNode> for PersistedNode {
    fn from(node: &CaseNode) -> Self {
        Self {
            id: node.id.clone(),
            case_id: node.case_id.clone(),
            node_type: node.node_type.as_str().to_string(),
            content: node.content.clone(),
            metadata: node.metadata.clone(),
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersistedEdge {
    pub id: String,
    pub case_id: String,
    pub source_node_id: String,
    pub target_node_id: String,
    pub edge_type: String,
    pub metadata: Value,
    pub created_at: String,
}

impl From<&CaseEdge> for PersistedEdge {
    fn from(edge: &CaseEdge) -> Self {
        Self {
            id: edge.id.clone(),
            case_id: edge.case_id.clone(),
            source_node_id: edge.source_node_id.clone(),
            target_node_id: edge.target_node_id.clone(),
            edge_type: edge.edge_type.as_str().to_string(),
            metadata: edge.metadata.clone(),
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}