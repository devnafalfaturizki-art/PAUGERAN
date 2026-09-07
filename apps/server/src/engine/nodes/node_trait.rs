use serde::{Deserialize, Serialize};
use crate::engine::context::ExecutionContext;
use crate::engine::EngineError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeResult {
    pub node_id: String,
    pub node_type: String,
    pub success: bool,
    pub output: NodeOutput,
    pub confidence: f32,
    pub warnings: Vec<String>,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum NodeOutput {
    Facts { facts: Vec<FactEntry> },
    Issues { issues: Vec<IssueEntry> },
    Rules { rules: Vec<RuleEntry> },
    Arguments { arguments: Vec<ArgumentEntry> },
    Counterarguments { counterarguments: Vec<CounterargumentEntry> },
    Risks { risks: Vec<RiskEntry> },
    Conclusions { conclusions: Vec<ConclusionEntry> },
    Clarification { questions: Vec<String> },
    CitationValidation { valid: bool, issues: Vec<String> },
    Requalification { challenges: Vec<RequalificationChallenge> },
    Empty { reason: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactEntry {
    pub id: String,
    pub content: String,
    pub source: String,
    pub verified: bool,
    pub certainty: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueEntry {
    pub id: String,
    pub description: String,
    pub legal_basis: Vec<String>,
    pub priority: IssuePriority,
    pub certainty: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuePriority {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleEntry {
    pub id: String,
    pub regulation: String,
    pub article: String,
    pub full_text: String,
    pub hierarchy_level: u8,
    pub status: RuleStatus,
    pub effective_date: String,
    pub source_url: Option<String>,
    pub access_date: String,
    pub precedential_weight: Option<f32>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuleStatus {
    Active,
    Repealed,
    Amended,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArgumentEntry {
    pub id: String,
    pub conclusion: String,
    pub premises: Vec<String>,
    pub supporting_rules: Vec<String>,
    pub supporting_facts: Vec<String>,
    pub strength: ArgumentStrength,
    pub certainty: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArgumentStrength {
    Strong,
    Moderate,
    Weak,
    Speculative,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterargumentEntry {
    pub id: String,
    pub target_argument_id: String,
    pub counter_conclusion: String,
    pub counter_premises: Vec<String>,
    pub attacking_rules: Vec<String>,
    pub strength: ArgumentStrength,
    pub likelihood: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskEntry {
    pub id: String,
    pub description: String,
    pub probability: f32,
    pub impact: RiskImpact,
    pub mitigation: String,
    pub residual_risk: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RiskImpact {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConclusionEntry {
    pub id: String,
    pub statement: String,
    pub certainty: f32,
    pub supporting_arguments: Vec<String>,
    pub conditions: Vec<String>,
    pub alternative_conclusions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequalificationChallenge {
    pub original_qualification: String,
    pub suggested_qualification: String,
    pub legal_basis: String,
    pub explanation: String,
}

#[async_trait::async_trait]
pub trait NodeExecutor: Send + Sync {
    async fn execute(&self, context: &mut ExecutionContext) -> Result<NodeResult, EngineError>;
    fn node_type(&self) -> &'static str;
}
