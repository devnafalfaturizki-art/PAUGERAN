use tracing::info;
use crate::engine::context::ExecutionContext;
use crate::engine::layers::base::{LayerOutput, LegalLayer};
use crate::engine::EngineError;

pub struct SociologicalInterpreter;

impl SociologicalInterpreter {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl LegalLayer for SociologicalInterpreter {
    async fn analyze(&self, context: &ExecutionContext) -> Result<LayerOutput, EngineError> {
        context.check_cancelled()?;
        info!(case_id=%context.case_id, layer=4, "sociological interpretation");

        let mut output = LayerOutput::new(4, "Sociological Interpretation");

        let facts: Vec<&str> = context.case_graph.facts.iter().map(|f| f.content.as_str()).collect();
        let issues: Vec<&str> = context.case_graph.issues.iter().map(|i| i.content.as_str()).collect();

        output.interpretation = format!(
            "Penafsiran sosiologis: Mempertimbangkan realitas sosial, budaya, ekonomi, dan dampak praktis. \
             Fakta: {}. Isu: {}.",
            facts.join("; ").chars().take(200).collect::<String>(),
            issues.join("; ").chars().take(200).collect::<String>()
        );

        output.key_findings = vec![
            "Konteks sosial dan budaya lokal dipertimbangkan".to_string(),
            "Realitas ekonomi para pihak dianalisis".to_string(),
            "Akses terhadap keadilan dievaluasi".to_string(),
            "Norma budaya dan hukum adat (jika relevan) diintegrasikan".to_string(),
        ];

        output.ambiguities = vec![
            "Realitas sosial dapat bervariasi antar daerah".to_string(),
        ];

        output.certainty = 0.6;
        output.supporting_sources = facts.iter().map(|f| f.to_string()).collect();

        Ok(output)
    }

    fn layer_number(&self) -> u8 {
        4
    }

    fn layer_name(&self) -> &'static str {
        "Sociological Interpretation"
    }
}
