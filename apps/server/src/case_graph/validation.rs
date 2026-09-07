use super::graph_manager::GraphError;

pub fn validate_edge_case(
    source_case: &str,
    target_case: &str,
    edge_case: &str,
) -> Result<(), GraphError> {
    if source_case == target_case && source_case == edge_case {
        Ok(())
    } else {
        Err(GraphError::CrossCaseEdge)
    }
}
