//! Database boundary. Repositories currently delegate to the SQLite persistence implementation.

pub mod models;
pub mod pool;
pub mod repositories;

pub use crate::persistence::{CaseEdgeRecord, CaseNodeRecord, CaseRecord, Database};
