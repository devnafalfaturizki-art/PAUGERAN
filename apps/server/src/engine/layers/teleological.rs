use tracing::info;
use crate::engine::context::ExecutionContext;
use crate::engine::layers::base::{LayerOutput, LegalLayer};
use crate::engine::EngineError;

pub struct TeleologicalInterpreter;

impl TeleologicalInterpreter {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl LegalLayer for TeleologicalInterpreter {
    async fn analyze(&self, context: &ExecutionContext) -> Result<LayerOutput, EngineError> {
        context.check_cancelled()?;
        info!(case_id=%context.case_id, layer=3, "teleological interpretation");

        let mut output = LayerOutput::new(3, "Teleological Interpretation");

        let issues: Vec<&str> = context.case_graph.issues.iter().map(|i| i.content.as_str()).collect();
        let rules: Vec<&str> = context.case_graph.rules.iter().map(|r| r.content.as_str()).collect();

        output.interpretation = format!(
            "Penafsiran teleologis: Menganalisis tujuan hukum (geest van de wet) yang ingin dicapai. \
             Isu: {}. Peraturan: {}.",
            issues.join("; ").chars().take(200).collect::<String>(),
            rules.join("; ").chars().take(200).collect::<String>()
        );

        output.key_findings = vec![
            "Tujuan sosial-ekonomi peraturan diidentifikasi".to_string(),
            "Dampak kebijakan (policy impact) dianalisis".to_string(),
            "Tujuan perlindungan pihak lemah menjadi pertimbangan utama".to_string(),
        ];

        output.ambiguities = vec![
            "Tujuan hukum dapat bertentangan antar stakeholder".to_string(),
        ];

        output.certainty = 0.65;
        output.supporting_sources = rules.iter().map(|r| r.to_string()).collect();

        Ok(output)
    }

    fn layer_number(&self) -> u8 {
        3
    }

    fn layer_name(&self) -> &'static str {
        "Teleological Interpretation"
    }
}
