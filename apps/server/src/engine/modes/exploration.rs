use tracing::info;
use crate::engine::context::ExecutionContext;
use crate::engine::nodes::{NodeExecutor, NodeResult};
use crate::engine::layers::base::{LegalLayer, LayerOutput};
use crate::engine::mode_router::ReasoningMode;
use crate::engine::state_machine::CaseState;
use crate::engine::EngineError;

pub struct ExplorationMode;

impl ExplorationMode {
    pub fn new() -> Self {
        Self
    }

    fn generate_clarifying_questions(&self, context: &ExecutionContext) -> Vec<String> {
        let mut questions = Vec::new();

        if context.case_graph.facts.is_empty() {
            questions.push("Ceritakan fakta utama yang terjadi dalam urutan waktu.".to_string());
        }

        if context.case_graph.issues.is_empty() {
            questions.push("Apa tujuan utama Anda dalam mencari analisis hukum ini?".to_string());
        }

        questions.push("Siapa pihak-pihak yang terlibat dan apa hubungan hukum antar mereka?".to_string());
        questions.push("Apakah ada dokumen atau bukti pendukung yang sudah dimiliki?".to_string());
        questions.push("Kapan kejadian utama terjadi dan apakah ada batas waktu yang perlu diperhatikan?".to_string());

        if context.conversation_history.len() < 3 {
            questions.push("Apakah ada fakta tambahan yang belum disampaikan?".to_string());
        }

        questions.truncate(5);
        questions
    }

    async fn run_exploration_layer(&self, context: &mut ExecutionContext) -> Result<Vec<LayerOutput>, EngineError> {
        let mut layers = Vec::new();

        let grammatical = crate::engine::engine::layers::grammatical::GrammaticalInterpreter::new();
        layers.push(grammatical.analyze(context).await?);

        let systematic = crate::engine::engine::layers::systematic::SystematicInterpreter::new();
        layers.push(systematic.analyze(context).await?);

        let teleological = crate::engine::engine::layers::teleological::TeleologicalInterpreter::new();
        layers.push(teleological.analyze(context).await?);

        layers
    }
}

#[async_trait::async_trait]
impl NodeExecutor for ExplorationMode {
    async fn execute(&self, context: &mut ExecutionContext) -> Result<NodeResult, EngineError> {
        context.check_cancelled()?;
        info!(case_id=%context.case_id, mode="exploration", "executing exploration mode");

        let questions = Self::generate_clarifying_questions(self, context);
        let layers = Self::run_exploration_layer(self, context).await?;

        let synthesis = crate::engine::engine::layers::synthesis::SynthesisEngine::new()
            .synthesize(&layers, context)?;

        let possible_issues: Vec<String> = context.case_graph.issues.iter().map(|i| i.content.clone()).collect();

        let interpretation = format!(
            "MODE EKSPLORASI — Analisis Kemungkinan Isu Hukum\n\
             ==============================================\n\n\
             Berdasarkan informasi yang telah dikumpulkan, berikut adalah kemungkinan isu hukum yang mungkin relevan:\n\n\
            {:?}\n\n\
             Catatan: Mode eksplorasi tidak memberikan kesimpulan definitif. \
             Analisis ini hanya mengidentifikasi kemungkinan dan memerlukan klarifikasi lebih lanjut. \
             Tingkat kepastian: {:.0}%\n\n\
             Pertanyaan klarifikasi yang disarankan:\n{}",
            possible_issues,
            synthesis.certainty * 100.0,
            questions.iter().enumerate().map(|(i, q)| format!("{}. {}", i + 1, q)).collect::<Vec<_>>().join("\n")
        );

        Ok(NodeResult {
            node_id: format!("exploration_{}", uuid::Uuid::new_v4()),
            node_type: "mode_exploration".to_string(),
            success: true,
            output: NodeOutput::Clarification { questions },
            confidence: synthesis.certainty,
            warnings: vec!["Masih memerlukan klarifikasi fakta lebih lanjut".to_string()],
            metadata: serde_json::json!({"interpretation": interpretation, "possible_issues": possible_issues, "certainty": synthesis.certainty}),
        })
    }

    fn node_type(&self) -> &'static str {
        "mode_exploration"
    }
}
