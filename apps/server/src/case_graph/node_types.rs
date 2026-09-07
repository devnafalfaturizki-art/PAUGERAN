use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeType {
    Case,
    Party,
    Fact,
    Evidence,
    Issue,
    Rule,
    Source,
    Argument,
    Counterargument,
    Risk,
    Conclusion,
    Document,
}

impl NodeType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Case => "case",
            Self::Party => "party",
            Self::Fact => "fact",
            Self::Evidence => "evidence",
            Self::Issue => "issue",
            Self::Rule => "rule",
            Self::Source => "source",
            Self::Argument => "argument",
            Self::Counterargument => "counterargument",
            Self::Risk => "risk",
            Self::Conclusion => "conclusion",
            Self::Document => "document",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CaseNode {
    pub id: String,
    pub case_id: String,
    pub node_type: NodeType,
    pub content: String,
    pub metadata: serde_json::Value,
}
