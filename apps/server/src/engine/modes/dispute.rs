use tracing::info;
use crate::engine::context::ExecutionContext;
use crate::engine::nodes::{NodeExecutor, NodeOutput, NodeResult};
use crate::engine::layers::base::{LegalLayer, LayerOutput};
use crate::engine::EngineError;

pub struct DisputeMode;

impl DisputeMode {
    pub fn new() -> Self {
        Self
    }

    fn build_position_mapping(&self, context: &ExecutionContext) -> (Vec<String>, Vec<String>) {
        let our_position: Vec<String> = context.case_graph.arguments.iter().map(|a| a.content.clone()).collect();
        let their_position: Vec<String> = context.case_graph.counterarguments.iter().map(|c| c.content.clone()).collect();
        (our_position, their_position)
    }

    fn build_settlement_options(&self, context: &ExecutionContext) -> Vec<String> {
        vec![
            "Negosiasi langsung antar pihak".to_string(),
            "Mediasi melalui mediator independen".to_string(),
            "Arbitrase (jika ada klausul arbitrase dalam perjanjian)".to_string(),
            "Putusan pengadilan (litigasi)".to_string(),
            "Penyelesaian di luar pengadilan (restorative justice)".to_string(),
        ]
    }

    async fn run_dispute_layers(&self, context: &mut ExecutionContext) -> Result<Vec<LayerOutput>, EngineError> {
        let mut layers = Vec::new();

        let grammatical = crate::engine::layers::grammatical::GrammaticalInterpreter::new();
        layers.push(grammatical.analyze(context).await?);

        let systematic = crate::engine::layers::systematic::SystematicInterpreter::new();
        layers.push(systematic.analyze(context).await?);

        let comparative = crate::engine::layers::comparative::ComparativeInterpreter::new();
        layers.push(comparative.analyze(context).await?);

        let critical = crate::engine::layers::critical::CriticalInterpreter::new();
        layers.push(critical.analyze(context).await?);

        Ok(layers)
    }
}

#[async_trait::async_trait]
impl NodeExecutor for DisputeMode {
    async fn execute(&self, context: &mut ExecutionContext) -> Result<NodeResult, EngineError> {
        context.check_cancelled()?;
        info!(case_id=%context.case_id, mode="dispute", "executing dispute mode");

        let layers = Self::run_dispute_layers(self, context).await?;
        let (our_position, their_position) = Self::build_position_mapping(self, context);
        let settlement_options = Self::build_settlement_options(self, context);

        let synthesis = crate::engine::layers::synthesis::SynthesisEngine::new()
            .synthesize(&layers, context)?;

        let interpretation = format!(
            "MODE SENGKETA — Pemetaan Posisi & Strategi Penyelesaian\n\
             ======================================================\n\n\
             POSISI KITA:\n{}\n\n\
             POSISI LAWAN:\n{}\n\n\
             OPSI PENYELESAIAN:\n{}\n\n\
             Tingkat kepastian: {:.0}%\n\n\
             Rekomendasi: Evaluasi kekuatan dan kelemahan masing-masing posisi sebelum memilih strategi penyelesaian.",
            our_position.iter().enumerate().map(|(i, p)| format!("{}. {}", i + 1, p)).collect::<Vec<_>>().join("\n"),
            their_position.iter().enumerate().map(|(i, p)| format!("{}. {}", i + 1, p)).collect::<Vec<_>>().join("\n"),
            settlement_options.iter().enumerate().map(|(i, o)| format!("{}. {}", i + 1, o)).collect::<Vec<_>>().join("\n"),
            synthesis.certainty * 100.0
        );

        Ok(NodeResult {
            node_id: format!("dispute_{}", uuid::Uuid::new_v4()),
            node_type: "mode_dispute".to_string(),
            success: true,
            output: NodeOutput::Clarification { questions: vec!["Apa tuntutan pihak lawan dan bukti yang sudah tersedia?".to_string()] },
            confidence: synthesis.certainty,
            warnings: Vec::new(),
            metadata: serde_json::json!({"interpretation": interpretation, "our_position": our_position, "their_position": their_position, "settlement_options": settlement_options, "certainty": synthesis.certainty}),
        })
    }

    fn node_type(&self) -> &'static str {
        "mode_dispute"
    }
}
