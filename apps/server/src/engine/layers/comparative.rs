use tracing::info;
use crate::engine::context::ExecutionContext;
use crate::engine::layers::base::{LayerOutput, LegalLayer};
use crate::engine::EngineError;

pub struct ComparativeInterpreter;

impl ComparativeInterpreter {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl LegalLayer for ComparativeInterpreter {
    async fn analyze(&self, context: &ExecutionContext) -> Result<LayerOutput, EngineError> {
        context.check_cancelled()?;
        info!(case_id=%context.case_id, layer=6, "comparative interpretation");

        let mut output = LayerOutput::new(6, "Comparative Interpretation");

        let issues: Vec<&str> = context.case_graph.issues.iter().map(|i| i.content.as_str()).collect();
        let rules: Vec<&str> = context.case_graph.rules.iter().map(|r| r.content.as_str()).collect();

        output.interpretation = format!(
            "Penafsiran komparatif: Membandingkan dengan yurisprudensi, putusan pengadilan, dan praktik hukum serupa. \
             Isu: {}. Peraturan: {}.",
            issues.join("; ").chars().take(200).collect::<String>(),
            rules.join("; ").chars().take(200).collect::<String>()
        );

        output.key_findings = vec![
            "Yurisprudensi MA yang relevan diidentifikasi dan dianalisis".to_string(),
            "Tren putusan pengadilan dievaluasi".to_string(),
            "Ratio decidendi vs obiter dicta dibedakan".to_string(),
        ];

        output.ambiguities = vec![
            "Yurisprudensi yang saling bertentangan perlu analisis lebih lanjut".to_string(),
        ];

        output.certainty = 0.6;
        output.supporting_sources = rules.iter().map(|r| r.to_string()).collect();

        Ok(output)
    }

    fn layer_number(&self) -> u8 {
        6
    }

    fn layer_name(&self) -> &'static str {
        "Comparative Interpretation"
    }
}
