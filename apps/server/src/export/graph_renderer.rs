use crate::error::AppError;

pub fn render_graph(nodes: &[crate::case_graph::CaseNode], edges: &[crate::case_graph::CaseEdge]) -> String {
    let mut output = String::new();
    output.push_str("CASE GRAPH\n");
    output.push_str(&"=".repeat(40));
    output.push('\n');
    output.push_str("Nodes:\n");
    for node in nodes {
        output.push_str(&format!("  - {} ({})\n", node.id, node.node_type.as_str()));
    }
    output.push_str("Edges:\n");
    for edge in edges {
        output.push_str(&format!("  - {} -> {} ({})\n", edge.source_node_id, edge.target_node_id, edge.edge_type.as_str()));
    }
    output
}
