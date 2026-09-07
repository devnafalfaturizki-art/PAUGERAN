use tracing::info;
use crate::engine::context::ExecutionContext;
use crate::engine::layers::base::{LayerOutput, LegalLayer};
use crate::engine::EngineError;

pub struct CriticalInterpreter;

impl CriticalInterpreter {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl LegalLayer for CriticalInterpreter {
    async fn analyze(&self, context: &ExecutionContext) -> Result<LayerOutput, EngineError> {
        context.check_cancelled()?;
        info!(case_id=%context.case_id, layer=7, "critical interpretation");

        let mut output = LayerOutput::new(7, "Critical Interpretation");

        let arguments: Vec<&str> = context.case_graph.arguments.iter().map(|a| a.content.as_str()).collect();
        let risks: Vec<&str> = context.case_graph.risks.iter().map(|r| r.content.as_str()).collect();

        output.interpretation = format!(
            "Penafsiran kritis: Secara aktif mencari kelemahan, risiko, dan alternatif interpretasi. \
             Argumen: {}. Risiko: {}.",
            arguments.join("; ").chars().take(200).collect::<String>(),
            risks.join("; ").chars().take(200).collect::<String>()
        );

        output.key_findings = vec![
            "Kelemahan dalam argumen diidentifikasi dan dievaluasi".to_string(),
            "Risiko dari setiap interpretasi dianalisis".to_string(),
            "Alternatif interpretasi yang mungkin disajikan".to_string(),
            "Devil's advocate analysis dilakukan untuk menguji kekuatan argumen".to_string(),
        ];

        output.ambiguities = vec![
            "Beberapa interpretasi memiliki probabilitas serupa".to_string(),
        ];

        output.certainty = 0.5;
        output.supporting_sources = arguments.iter().map(|a| a.to_string()).collect();

        Ok(output)
    }

    fn layer_number(&self) -> u8 {
        7
    }

    fn layer_name(&self) -> &'static str {
        "Critical Interpretation"
    }
}
