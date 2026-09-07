//! Visualization payload generator. Produces the JSON consumed by the
//! Cytoscape.js viewer in the frontend.

use serde::{Deserialize, Serialize};

use super::edge_types::CaseEdge;
use super::node_types::CaseNode;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VisualizationNode {
    pub id: String,
    pub label: String,
    pub node_type: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VisualizationEdge {
    pub id: String,
    pub source: String,
    pub target: String,
    pub edge_type: String,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VisualizationGraph {
    pub case_id: String,
    pub nodes: Vec<VisualizationNode>,
    pub edges: Vec<VisualizationEdge>,
}

pub struct Visualization;

impl Visualization {
    pub fn build(case_id: &str, nodes: &[CaseNode], edges: &[CaseEdge]) -> VisualizationGraph {
        let vis_nodes = nodes
            .iter()
            .filter(|node| node.case_id == case_id)
            .map(|node| VisualizationNode {
                id: node.id.clone(),
                label: node.content.chars().take(60).collect::<String>(),
                node_type: node.node_type.as_str().to_string(),
                content: node.content.clone(),
            })
            .collect();

        let vis_edges = edges
            .iter()
            .filter(|edge| edge.case_id == case_id)
            .map(|edge| VisualizationEdge {
                id: edge.id.clone(),
                source: edge.source_node_id.clone(),
                target: edge.target_node_id.clone(),
                edge_type: edge.edge_type.as_str().to_string(),
                label: edge.edge_type.as_str().to_string(),
            })
            .collect();

        VisualizationGraph {
            case_id: case_id.to_string(),
            nodes: vis_nodes,
            edges: vis_edges,
        }
    }
}