use tracing::info;
use crate::engine::context::ExecutionContext;
use crate::engine::nodes::{NodeExecutor, NodeOutput, NodeResult};
use crate::engine::layers::base::{LegalLayer, LayerOutput};
use crate::engine::EngineError;

pub struct NeutralMode;

impl NeutralMode {
    pub fn new() -> Self {
        Self
    }

    async fn simulate_judicial_review(&self, context: &ExecutionContext) -> Vec<String> {
        let mut simulation = Vec::new();

        simulation.push("PENILAIAN HAKIM (SIMULASI NEUTRAL)".to_string());
        simulation.push("=====================================".to_string());
        simulation.push("".to_string());
        simulation.push("Analisis bukti secara simetris untuk kedua belah pihak.".to_string());
        simulation.push("".to_string());

        let facts: Vec<&str> = context.case_graph.facts.iter().map(|f| f.content.as_str()).collect();
        let rules: Vec<&str> = context.case_graph.rules.iter().map(|r| r.content.as_str()).collect();

        if !facts.is_empty() {
            simulation.push("Bukti yang diajukan:".to_string());
            for fact in facts.iter().take(5) {
                simulation.push(format!("  - {}", fact));
            }
        }

        if !rules.is_empty() {
            simulation.push("".to_string());
            simulation.push("Dasar hukum yang relevan:".to_string());
            for rule in rules.iter().take(5) {
                simulation.push(format!("  - {}", rule));
            }
        }

        simulation.push("".to_string());
        simulation.push("Penimbangan:".to_string());
        simulation.push("  - Kekuatan bukti objektif vs bukti lisan".to_string());
        simulation.push("  - Konsistensi testimoni dan dokumen".to_string());
        simulation.push("  - Kredibilitas saksi dan obyektivitas bukti".to_string());
        simulation.push("".to_string());
        simulation.push("Kesimpulan simulasi:".to_string());
        simulation.push("  Hasil dapat bervariasi tergantung pada penilaian hakim.".to_string());

        simulation
    }

    async fn run_neutral_layers(&self, context: &mut ExecutionContext) -> Result<Vec<LayerOutput>, EngineError> {
        let mut layers = Vec::new();

        let grammatical = crate::engine::layers::grammatical::GrammaticalInterpreter::new();
        layers.push(grammatical.analyze(context).await?);

        let systematic = crate::engine::layers::systematic::SystematicInterpreter::new();
        layers.push(systematic.analyze(context).await?);

        let teleological = crate::engine::layers::teleological::TeleologicalInterpreter::new();
        layers.push(teleological.analyze(context).await?);

        let comparative = crate::engine::layers::comparative::ComparativeInterpreter::new();
        layers.push(comparative.analyze(context).await?);

        let critical = crate::engine::layers::critical::CriticalInterpreter::new();
        layers.push(critical.analyze(context).await?);

        Ok(layers)
    }
}

#[async_trait::async_trait]
impl NodeExecutor for NeutralMode {
    async fn execute(&self, context: &mut ExecutionContext) -> Result<NodeResult, EngineError> {
        context.check_cancelled()?;
        info!(case_id=%context.case_id, mode="neutral", "executing neutral/judicial mode");

        let layers = Self::run_neutral_layers(self, context).await?;
        let simulation = Self::simulate_judicial_review(self, context).await;

        let synthesis = crate::engine::layers::synthesis::SynthesisEngine::new()
            .synthesize(&layers, context)?;

        let interpretation = format!(
            "MODE NETRAL — Simulasi Judicial Review\n\
             =====================================\n\n{}\n\n\
             Tingkat kepastian: {:.0}%\n\n\
             Rekomendasi: Simulasi ini memberikan perspektif netral untuk membantu \
             mengevaluasi kekuatan dan kelemahan posisi hukum sebelum mengajukan ke pengadilan.",
            simulation.join("\n"),
            synthesis.certainty * 100.0
        );

        Ok(NodeResult {
            node_id: format!("neutral_{}", uuid::Uuid::new_v4()),
            node_type: "mode_neutral".to_string(),
            success: true,
            output: NodeOutput::Clarification { questions: vec!["Bukti apa yang mendukung masing-masing pihak?".to_string()] },
            confidence: synthesis.certainty,
            warnings: Vec::new(),
            metadata: serde_json::json!({"interpretation": interpretation, "simulation": simulation, "certainty": synthesis.certainty}),
        })
    }

    fn node_type(&self) -> &'static str {
        "mode_neutral"
    }
}
