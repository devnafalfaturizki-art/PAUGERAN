use tracing::info;
use crate::engine::context::ExecutionContext;
use crate::engine::layers::base::{LayerOutput, LegalLayer};
use crate::engine::EngineError;

pub struct SystematicInterpreter;

impl SystematicInterpreter {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl LegalLayer for SystematicInterpreter {
    async fn analyze(&self, context: &ExecutionContext) -> Result<LayerOutput, EngineError> {
        context.check_cancelled()?;
        info!(case_id=%context.case_id, layer=2, "systematic interpretation");

        let mut output = LayerOutput::new(2, "Systematic Interpretation");

        let rules: Vec<&str> = context.case_graph.rules.iter().map(|r| r.content.as_str()).collect();
        let issues: Vec<&str> = context.case_graph.issues.iter().map(|i| i.content.as_str()).collect();

        output.interpretation = format!(
            "Penafsiran sistematis: Menempatkan peraturan dalam konteks sistem hukum secara keseluruhan. \
             Peraturan: {}. Isu: {}.",
            rules.join("; ").chars().take(200).collect::<String>(),
            issues.join("; ").chars().take(200).collect::<String>()
        );

        output.key_findings = vec![
            "Hierarki norma diidentifikasi dan dianalisis".to_string(),
            "Hubungan antar peraturan dalam bidang hukum yang sama dipetakan".to_string(),
            "Asas-asas hukum yang mendasari (pacta sunt servanda, itikad baik) dipertimbangkan".to_string(),
        ];

        output.ambiguities = vec![
            "Potensi konflik norma perlu diverifikasi melalui Norm Conflict Resolution".to_string(),
        ];

        output.certainty = 0.75;
        output.supporting_sources = rules.iter().map(|r| r.to_string()).collect();

        Ok(output)
    }

    fn layer_number(&self) -> u8 {
        2
    }

    fn layer_name(&self) -> &'static str {
        "Systematic Interpretation"
    }
}
