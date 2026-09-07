//! Case Graph domain and traversal primitives.

pub mod edge_types;
pub mod graph_manager;
pub mod node_types;
pub mod persistence;
pub mod traversal;
pub mod validation;
pub mod visualization;

pub use edge_types::{CaseEdge, EdgeType};
pub use graph_manager::GraphManager;
pub use node_types::{CaseNode, NodeType};
