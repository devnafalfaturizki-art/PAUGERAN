//! Case Graph traversal helpers. Exposes BFS/DFS walks over the
//! in-memory graph used by the engine to render conclusions and
//! evidence chains.

use petgraph::{
    graph::NodeIndex,
    visit::{Bfs, Dfs},
    Direction,
};
use serde::{Deserialize, Serialize};

use super::{CaseEdge, CaseNode, EdgeType, NodeType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraversalStep {
    pub node_id: String,
    pub node_type: String,
    pub content: String,
    pub edge_type: Option<String>,
    pub depth: usize,
}

pub struct Traversal;

impl Traversal {
    pub fn collect_support_chain(
        graph: &petgraph::Graph<CaseNode, CaseEdge>,
        start: NodeIndex,
    ) -> Vec<TraversalStep> {
        let mut steps = Vec::new();
        let mut dfs = Dfs::new(graph, start);
        let mut depth: std::collections::HashMap<NodeIndex, usize> = std::collections::HashMap::new();
        depth.insert(start, 0);
        while let Some(next) = dfs.next(graph) {
            let node = &graph[next];
            steps.push(TraversalStep {
                node_id: node.id.clone(),
                node_type: node.node_type.as_str().to_string(),
                content: node.content.clone(),
                edge_type: None,
                depth: *depth.get(&next).unwrap_or(&0),
            });
            for neighbour in graph.neighbors_directed(next, Direction::Outgoing) {
                depth.entry(neighbour).or_insert(*depth.get(&next).unwrap_or(&0) + 1);
            }
        }
        steps
    }

    pub fn collect_descendants(
        graph: &petgraph::Graph<CaseNode, CaseEdge>,
        start: NodeIndex,
    ) -> Vec<TraversalStep> {
        let mut steps = Vec::new();
        let mut bfs = Bfs::new(graph, start);
        while let Some(next) = bfs.next(graph) {
            let node = &graph[next];
            steps.push(TraversalStep {
                node_id: node.id.clone(),
                node_type: node.node_type.as_str().to_string(),
                content: node.content.clone(),
                edge_type: None,
                depth: 0,
            });
        }
        steps
    }

    pub fn node_type_count(
        graph: &petgraph::Graph<CaseNode, CaseEdge>,
        node_type: NodeType,
    ) -> usize {
        graph
            .node_indices()
            .filter(|index| graph[*index].node_type == node_type)
            .count()
    }

    pub fn edge_type_count(
        graph: &petgraph::Graph<CaseNode, CaseEdge>,
        edge_type: EdgeType,
    ) -> usize {
        graph
            .edge_indices()
            .filter(|index| graph[*index].edge_type == edge_type)
            .count()
    }
}