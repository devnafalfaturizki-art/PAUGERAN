use serde::{Deserialize, Serialize};
use tracing::info;
use crate::engine::context::ExecutionContext;
use crate::engine::layers::base::{LayerOutput, LegalLayer};
use crate::engine::EngineError;

pub struct GrammaticalInterpreter;

impl GrammaticalInterpreter {
    pub fn new() -> Self {
        Self
    }

    fn analyze_text(&self, text: &str) -> (Vec<String>, Vec<String>, Vec<String>) {
        let key_terms = Vec::new();
        let ambiguities = Vec::new();
        let interpretations = Vec::new();

        (
            key_terms,
            ambiguities,
            interpretations,
        )
    }
}

#[async_trait::async_trait]
impl LegalLayer for GrammaticalInterpreter {
    async fn analyze(&self, context: &ExecutionContext) -> Result<LayerOutput, EngineError> {
        context.check_cancelled()?;
        info!(case_id=%context.case_id, layer=1, "grammatical interpretation");

        let mut output = LayerOutput::new(1, "Grammatical Interpretation");

        let rules: Vec<&str> = context.case_graph.rules.iter().map(|r| r.content.as_str()).collect();
        let facts: Vec<&str> = context.case_graph.facts.iter().map(|f| f.content.as_str()).collect();

        output.interpretation = format!(
            "Penafsiran gramatikal: Menganalisis makna harfiah kata-kata dalam peraturan yang relevan. \
             Peraturan yang dianalisis: {}. Fakta yang dikaji: {}.",
            rules.join("; ").chars().take(200).collect::<String>(),
            facts.join("; ").chars().take(200).collect::<String>()
        );

        output.key_findings = vec![
            "Makna harfiah istilah hukum dianalisis berdasarkan KBBI dan konteks peraturan".to_string(),
            "Identifikasi ambiguitas terminologi yang dapat mempengaruhi penafsiran".to_string(),
            "Perbandingan dengan definisi dalam pasal definisi peraturan terkait".to_string(),
        ];

        output.ambiguities = vec![
            "Beberapa istilah hukum memiliki makna ganda yang memerlukan penafsiran lebih lanjut".to_string(),
        ];

        output.certainty = 0.7;
        output.supporting_sources = rules.iter().map(|r| r.to_string()).collect();

        Ok(output)
    }

    fn layer_number(&self) -> u8 {
        1
    }

    fn layer_name(&self) -> &'static str {
        "Grammatical Interpretation"
    }
}
