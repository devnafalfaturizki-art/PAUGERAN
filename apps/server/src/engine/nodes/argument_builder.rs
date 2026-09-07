use tracing::info;
use crate::engine::context::ExecutionContext;
use crate::engine::nodes::{NodeExecutor, NodeOutput, NodeResult, ArgumentEntry, ArgumentStrength};
use crate::engine::EngineError;

pub struct ArgumentBuilder;

impl ArgumentBuilder {
    pub fn new() -> Self {
        Self
    }

    fn build_arguments(&self, context: &ExecutionContext) -> Vec<ArgumentEntry> {
        let mut arguments = Vec::new();
        let facts: Vec<&str> = context.case_graph.facts.iter().map(|f| f.content.as_str()).collect();
        let rules: Vec<&str> = context.case_graph.rules.iter().map(|r| r.content.as_str()).collect();

        if !rules.is_empty() && !facts.is_empty() {
            arguments.push(ArgumentEntry {
                id: format!("arg_{}", uuid::Uuid::new_v4()),
                conclusion: "Berdasarkan fakta dan peraturan yang berlaku, posisi hukum pengguna memiliki dasar yang kuat.".to_string(),
                premises: facts.iter().take(3).cloned().map(String::from).collect(),
                supporting_rules: rules.iter().take(3).cloned().map(String::from).collect(),
                supporting_facts: facts.iter().take(3).cloned().map(String::from).collect(),
                strength: ArgumentStrength::Moderate,
                certainty: 0.65,
            });
        }

        if context.reasoning_mode == crate::engine::mode_router::ReasoningMode::Adversarial {
            arguments.push(ArgumentEntry {
                id: format!("arg_{}", uuid::Uuid::new_v4()),
                conclusion: "Argumen ini harus diuji melalui adversarial review untuk memastikan kekuatan".to_string(),
                premises: vec!["Semua argumen perlu stress test".to_string()],
                supporting_rules: rules.iter().take(2).cloned().map(String::from).collect(),
                supporting_facts: facts.iter().take(2).cloned().map(String::from).collect(),
                strength: ArgumentStrength::Moderate,
                certainty: 0.5,
            });
        }

        arguments
    }
}

#[async_trait::async_trait]
impl NodeExecutor for ArgumentBuilder {
    async fn execute(&self, context: &mut ExecutionContext) -> Result<NodeResult, EngineError> {
        context.check_cancelled()?;
        info!(case_id=%context.case_id, "building legal arguments");

        let arguments = Self::build_arguments(self, context);

        for arg in &arguments {
            context.case_graph.arguments.push(crate::engine::context::GraphNode {
                id: arg.id.clone(),
                node_type: "argument".to_string(),
                content: arg.conclusion.clone(),
                metadata: serde_json::json!({"strength": format!("{:?}", arg.strength).to_lowercase(), "certainty": arg.certainty}),
                certainty: arg.certainty,
            });
        }

        Ok(NodeResult {
            node_id: format!("arg_build_{}", uuid::Uuid::new_v4()),
            node_type: "argument_builder".to_string(),
            success: true,
            output: NodeOutput::Arguments { arguments: arguments.clone() },
            confidence: 0.65,
            warnings: Vec::new(),
            metadata: serde_json::json!({"total_arguments": arguments.len()}),
        })
    }

    fn node_type(&self) -> &'static str {
        "argument_builder"
    }
}
