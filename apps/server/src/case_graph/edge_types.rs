use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EdgeType {
    Supports,
    Challenges,
    GovernedBy,
    DerivedFrom,
    UncertainBecause,
    SupportedBy,
    ReliesOn,
    SourcedFrom,
}

impl EdgeType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Supports => "supports",
            Self::Challenges => "challenges",
            Self::GovernedBy => "governed_by",
            Self::DerivedFrom => "derived_from",
            Self::UncertainBecause => "uncertain_because",
            Self::SupportedBy => "supported_by",
            Self::ReliesOn => "relies_on",
            Self::SourcedFrom => "sourced_from",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CaseEdge {
    pub id: String,
    pub case_id: String,
    pub source_node_id: String,
    pub target_node_id: String,
    pub edge_type: EdgeType,
    pub metadata: serde_json::Value,
}
