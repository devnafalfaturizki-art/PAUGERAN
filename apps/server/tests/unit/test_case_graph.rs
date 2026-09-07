//! Unit tests for Case Graph functionality.
//! 
//! [CB §10] — Testing Strategy

#[cfg(test)]
mod tests {
    use paugeran::case_graph::{CaseGraph, CaseNode, CaseEdge, NodeType, EdgeType};

    #[test]
    fn test_create_node() {
        let node = CaseNode::new(
            "case-1".to_string(),
            NodeType::Fact,
            "Test fact".to_string(),
        );
        assert_eq!(node.case_id, "case-1");
        assert_eq!(node.node_type, NodeType::Fact);
        assert_eq!(node.content, "Test fact");
    }

    #[test]
    fn test_create_edge() {
        let edge = CaseEdge::new(
            "case-1".to_string(),
            "node-1".to_string(),
            "node-2".to_string(),
            EdgeType::Supports,
        );
        assert_eq!(edge.case_id, "case-1");
        assert_eq!(edge.source_node_id, "node-1");
        assert_eq!(edge.target_node_id, "node-2");
        assert_eq!(edge.edge_type, EdgeType::Supports);
    }

    #[test]
    fn test_graph_cycle_detection() {
        let mut graph = CaseGraph::new();
        let node1 = graph.add_node("case-1", NodeType::Fact, "Fact 1".to_string());
        let node2 = graph.add_node("case-1", NodeType::Issue, "Issue 1".to_string());
        graph.add_edge(node1, node2, EdgeType::DerivedFrom);
        
        assert!(!graph.has_cycle());
    }
}
