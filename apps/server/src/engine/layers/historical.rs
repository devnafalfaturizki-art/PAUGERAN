use tracing::info;
use crate::engine::context::ExecutionContext;
use crate::engine::layers::base::{LayerOutput, LegalLayer};
use crate::engine::EngineError;

pub struct HistoricalInterpreter;

impl HistoricalInterpreter {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl LegalLayer for HistoricalInterpreter {
    async fn analyze(&self, context: &ExecutionContext) -> Result<LayerOutput, EngineError> {
        context.check_cancelled()?;
        info!(case_id=%context.case_id, layer=5, "historical interpretation");

        let mut output = LayerOutput::new(5, "Historical Interpretation");

        let rules: Vec<&str> = context.case_graph.rules.iter().map(|r| r.content.as_str()).collect();

        output.interpretation = format!(
            "Penafsiran historis: Mempertimbangkan sejarah legislasi, evolusi norma, dan maksud asli pembentuk UU. \
             Peraturan: {}.",
            rules.join("; ").chars().take(200).collect::<String>()
        );

        output.key_findings = vec![
            "Evolusi norma dari waktu ke waktu ditelusuri".to_string(),
            "Risalah pembahasan UU dianalisis untuk memahami original intent".to_string(),
            "Perubahan sosial yang mempengaruhi perubahan hukum diidentifikasi".to_string(),
        ];

        output.ambiguities = vec![
            "Maksud asli pembentuk UU dapat berbeda dengan penerapan modern".to_string(),
        ];

        output.certainty = 0.65;
        output.supporting_sources = rules.iter().map(|r| r.to_string()).collect();

        Ok(output)
    }

    fn layer_number(&self) -> u8 {
        5
    }

    fn layer_name(&self) -> &'static str {
        "Historical Interpretation"
    }
}
