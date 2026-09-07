use petgraph::{
    graph::{Graph, NodeIndex},
    Direction,
};
use thiserror::Error;

use super::{CaseEdge, CaseNode};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum GraphError {
    #[error("node tidak ditemukan: {0}")]
    MissingNode(String),
    #[error("edge tidak boleh menghubungkan node yang berbeda perkara")]
    CrossCaseEdge,
}

#[derive(Default)]
pub struct GraphManager {
    graph: Graph<CaseNode, CaseEdge>,
}

impl GraphManager {
    pub fn add_node(&mut self, node: CaseNode) -> NodeIndex {
        self.graph.add_node(node)
    }

    pub fn add_edge(&mut self, edge: CaseEdge) -> Result<(), GraphError> {
        let source = self.find_node(&edge.source_node_id)?;
        let target = self.find_node(&edge.target_node_id)?;
        let source_case = &self.graph[source].case_id;
        if source_case != &self.graph[target].case_id || source_case != &edge.case_id {
            return Err(GraphError::CrossCaseEdge);
        }
        self.graph.add_edge(source, target, edge);
        Ok(())
    }

    pub fn node(&self, id: &str) -> Option<&CaseNode> {
        self.graph
            .node_indices()
            .find_map(|index| (self.graph[index].id == id).then_some(&self.graph[index]))
    }

    pub fn outgoing(&self, id: &str) -> Result<Vec<&CaseNode>, GraphError> {
        let node = self.find_node(id)?;
        Ok(self
            .graph
            .neighbors_directed(node, Direction::Outgoing)
            .map(|index| &self.graph[index])
            .collect())
    }

    fn find_node(&self, id: &str) -> Result<NodeIndex, GraphError> {
        self.graph
            .node_indices()
            .find(|index| self.graph[*index].id == id)
            .ok_or_else(|| GraphError::MissingNode(id.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::{GraphError, GraphManager};
    use crate::case_graph::{CaseEdge, CaseNode, EdgeType, NodeType};

    fn node(id: &str, case_id: &str) -> CaseNode {
        CaseNode {
            id: id.to_string(),
            case_id: case_id.to_string(),
            node_type: NodeType::Fact,
            content: id.to_string(),
            metadata: serde_json::json!({}),
        }
    }

    #[test]
    fn rejects_edges_between_different_cases() {
        let mut graph = GraphManager::default();
        graph.add_node(node("a", "case-a"));
        graph.add_node(node("b", "case-b"));
        let edge = CaseEdge {
            id: "edge".to_string(),
            case_id: "case-a".to_string(),
            source_node_id: "a".to_string(),
            target_node_id: "b".to_string(),
            edge_type: EdgeType::Supports,
            metadata: serde_json::json!({}),
        };
        assert_eq!(graph.add_edge(edge), Err(GraphError::CrossCaseEdge));
    }
}
