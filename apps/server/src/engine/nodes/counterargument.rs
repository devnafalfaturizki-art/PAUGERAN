use tracing::info;
use crate::engine::context::ExecutionContext;
use crate::engine::nodes::{NodeExecutor, NodeOutput, NodeResult, CounterargumentEntry, ArgumentStrength};
use crate::engine::EngineError;

pub struct CounterargumentGenerator;

impl CounterargumentGenerator {
    pub fn new() -> Self {
        Self
    }

    fn generate_counterarguments(&self, context: &ExecutionContext) -> Vec<CounterargumentEntry> {
        let mut counterarguments = Vec::new();

        for arg in &context.case_graph.arguments {
            let counter_id = format!("counter_{}", uuid::Uuid::new_v4());
            counterarguments.push(CounterargumentEntry {
                id: counter_id,
                target_argument_id: arg.id.clone(),
                counter_conclusion: format!("Kontraargumen terhadap: {}", arg.content),
                counter_premises: vec![
                    "Bukti yang digunakan mungkin tidak cukup".to_string(),
                    "Interpretasi hukum yang berbeda mungkin berlaku".to_string(),
                    "Fakta tambahan dapat mengubah posisi hukum".to_string(),
                ],
                attacking_rules: Vec::new(),
                strength: ArgumentStrength::Moderate,
                likelihood: 0.4,
            });
        }

        if counterarguments.is_empty() {
            counterarguments.push(CounterargumentEntry {
                id: format!("counter_{}", uuid::Uuid::new_v4()),
                target_argument_id: "general".to_string(),
                counter_conclusion: "Argumen ini memerlukan pembuktian lebih lanjut dan dapat ditentang oleh pihak lawan".to_string(),
                counter_premises: vec![
                    "Bukti belum terverifikasi secara menyeluruh".to_string(),
                    "Ada kemungkinan interpretasi hukum yang berlawanan".to_string(),
                    "Pihak lawan dapat mengajukan argumen serupa".to_string(),
                ],
                attacking_rules: Vec::new(),
                strength: ArgumentStrength::Weak,
                likelihood: 0.5,
            });
        }

        counterarguments
    }
}

#[async_trait::async_trait]
impl NodeExecutor for CounterargumentGenerator {
    async fn execute(&self, context: &mut ExecutionContext) -> Result<NodeResult, EngineError> {
        context.check_cancelled()?;
        info!(case_id=%context.case_id, "generating counterarguments");

        let counterarguments = Self::generate_counterarguments(self, context);
        let total_counterarguments = counterarguments.len();

        for counter in &counterarguments {
            context.case_graph.counterarguments.push(crate::engine::context::GraphNode {
                id: counter.id.clone(),
                node_type: "counterargument".to_string(),
                content: counter.counter_conclusion.clone(),
                metadata: serde_json::json!({"target": counter.target_argument_id, "likelihood": counter.likelihood}),
                certainty: counter.likelihood,
            });
        }

        Ok(NodeResult {
            node_id: format!("counter_gen_{}", uuid::Uuid::new_v4()),
            node_type: "counterargument".to_string(),
            success: true,
            output: NodeOutput::Counterarguments { counterarguments: counterarguments.clone() },
            confidence: 0.55,
            warnings: Vec::new(),
            metadata: serde_json::json!({"total_counterarguments": total_counterarguments}),
        })
    }

    fn node_type(&self) -> &'static str {
        "counterargument"
    }
}
