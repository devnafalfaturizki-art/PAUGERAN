use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionContext {
    pub case_id: String,
    pub user_id: String,
    pub reasoning_mode: crate::engine::mode_router::ReasoningMode,
    pub case_state: crate::engine::state_machine::CaseState,
    pub user_message: String,
    pub conversation_history: Vec<MessageTurn>,
    pub case_graph: CaseGraphSnapshot,
    pub knowledge_base_entries: Vec<String>,
    pub cancellation_token: CancellationToken,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageTurn {
    pub role: MessageRole,
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CaseGraphSnapshot {
    pub facts: Vec<GraphNode>,
    pub issues: Vec<GraphNode>,
    pub rules: Vec<GraphNode>,
    pub arguments: Vec<GraphNode>,
    pub counterarguments: Vec<GraphNode>,
    pub risks: Vec<GraphNode>,
    pub conclusions: Vec<GraphNode>,
    pub sources: Vec<GraphNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
    pub node_type: String,
    pub content: String,
    pub metadata: serde_json::Value,
    pub certainty: f32,
}

impl Default for ExecutionContext {
    fn default() -> Self {
        Self {
            case_id: String::new(),
            user_id: String::new(),
            reasoning_mode: crate::engine::mode_router::ReasoningMode::Exploration,
            case_state: crate::engine::state_machine::CaseState::Unknown,
            user_message: String::new(),
            conversation_history: Vec::new(),
            case_graph: CaseGraphSnapshot::default(),
            knowledge_base_entries: Vec::new(),
            cancellation_token: CancellationToken::default(),
        }
    }
}

impl ExecutionContext {
    pub fn new(
        case_id: impl Into<String>,
        user_id: impl Into<String>,
        reasoning_mode: crate::engine::mode_router::ReasoningMode,
        case_state: crate::engine::state_machine::CaseState,
        user_message: impl Into<String>,
    ) -> Self {
        Self {
            case_id: case_id.into(),
            user_id: user_id.into(),
            reasoning_mode,
            case_state,
            user_message: user_message.into(),
            conversation_history: Vec::new(),
            case_graph: CaseGraphSnapshot::default(),
            knowledge_base_entries: Vec::new(),
            cancellation_token: CancellationToken::default(),
        }
    }

    pub fn with_history(mut self, history: Vec<MessageTurn>) -> Self {
        self.conversation_history = history;
        self
    }

    pub fn with_graph(mut self, graph: CaseGraphSnapshot) -> Self {
        self.case_graph = graph;
        self
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancellation_token.is_cancelled()
    }

    pub fn check_cancelled(&self) -> Result<(), EngineError> {
        if self.is_cancelled() {
            return Err(EngineError::Cancelled);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CancellationToken {
    pub cancelled: bool,
}

impl Serialize for CancellationToken {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.cancelled.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CancellationToken {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let cancelled = bool::deserialize(deserializer)?;
        Ok(Self { cancelled })
    }
}

impl CancellationToken {
    pub fn cancel(&mut self) {
        self.cancelled = true;
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }
}

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("reasoning process was cancelled")]
    Cancelled,
    #[error("node execution failed: {0}")]
    NodeExecution(String),
    #[error("invalid reasoning mode for current state")]
    InvalidModeTransition,
    #[error("case graph integrity violation: {0}")]
    GraphIntegrity(String),
    #[error("citation validation failed: {0}")]
    CitationValidation(String),
    #[error("procedural check failed: {0}")]
    ProceduralCheck(String),
    #[error("internal engine error: {0}")]
    Internal(String),
}
