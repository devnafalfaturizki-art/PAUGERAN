use tracing::info;
use crate::engine::context::ExecutionContext;
use crate::engine::nodes::{NodeExecutor, NodeOutput, NodeResult, RiskEntry, RiskImpact};
use crate::engine::layers::base::{LegalLayer, LayerOutput};
use crate::engine::EngineError;

pub struct PreventiveMode;

impl PreventiveMode {
    pub fn new() -> Self {
        Self
    }

    fn build_risk_matrix(&self, context: &ExecutionContext) -> Vec<(String, String, String)> {
        let risks: Vec<&crate::engine::context::GraphNode> = context.case_graph.risks.iter().collect();
        risks.iter().map(|r| {
            let desc = r.content.clone();
            let mitigation = r.metadata.get("mitigation").and_then(|v| v.as_str()).unwrap_or("Perlu evaluasi lebih lanjut").to_string();
            let status = "Direncanakan".to_string();
            (desc, mitigation, status)
        }).collect()
    }

    fn build_compliance_checklist(&self, context: &ExecutionContext) -> Vec<String> {
        let mut checklist = Vec::new();

        let rules: Vec<&str> = context.case_graph.rules.iter().map(|r| r.content.as_str()).collect();
        let issues: Vec<&str> = context.case_graph.issues.iter().map(|i| i.content.as_str()).collect();

        checklist.push(format!("Verifikasi kepatuhan terhadap: {}", rules.join(", ")));
        checklist.push(format!("Identifikasi isu hukum: {}", issues.join(", ")));
        checklist.push("Review klausul-kausul dalam dokumen terkait".to_string());
        checklist.push("Evaluasi potensi konflik norma".to_string());
        checklist.push("Periksa batas waktu (daluwarsa) yang mungkin berlaku".to_string());
        checklist.push("Konsultasi dengan regulator atau otoritas terkait jika diperlukan".to_string());
        checklist.push("Dokumentasikan temuan untuk audit trail".to_string());

        checklist
    }

    async fn run_preventive_layers(&self, context: &mut ExecutionContext) -> Result<Vec<LayerOutput>, EngineError> {
        let mut layers = Vec::new();

        let grammatical = crate::engine::layers::grammatical::GrammaticalInterpreter::new();
        layers.push(grammatical.analyze(context).await?);

        let systematic = crate::engine::layers::systematic::SystematicInterpreter::new();
        layers.push(systematic.analyze(context).await?);

        let teleological = crate::engine::layers::teleological::TeleologicalInterpreter::new();
        layers.push(teleological.analyze(context).await?);

        let sociological = crate::engine::layers::sociological::SociologicalInterpreter::new();
        layers.push(sociological.analyze(context).await?);

        Ok(layers)
    }
}

#[async_trait::async_trait]
impl NodeExecutor for PreventiveMode {
    async fn execute(&self, context: &mut ExecutionContext) -> Result<NodeResult, EngineError> {
        context.check_cancelled()?;
        info!(case_id=%context.case_id, mode="preventive", "executing preventive mode");

        let layers = Self::run_preventive_layers(self, context).await?;
        let risk_matrix = Self::build_risk_matrix(self, context);
        let checklist = Self::build_compliance_checklist(self, context);

        let synthesis = crate::engine::layers::synthesis::SynthesisEngine::new()
            .synthesize(&layers, context)?;

        let interpretation = format!(
            "MODE PREVENTIF — Analisis Risiko & Mitigasi\n\
             ==========================================\n\n\
             MATRIK RISIKO:\n{}\n\n\
             CHECKLIST TINDAKAN PREVENTIF:\n{}\n\n\
             Tingkat kepastian: {:.0}%\n\n\
             Rekomendasi: Lakukan mitigasi risiko prioritas tinggi sebelum masalah berkembang menjadi sengketa.",
            risk_matrix.iter().enumerate().map(|(i, (d, m, s))| format!("{}. Risiko: {}\n   Mitigasi: {}\n   Status: {}", i + 1, d, m, s)).collect::<Vec<_>>().join("\n\n"),
            checklist.iter().enumerate().map(|(i, c)| format!("{}. {}", i + 1, c)).collect::<Vec<_>>().join("\n"),
            synthesis.certainty * 100.0
        );

        Ok(NodeResult {
            node_id: format!("preventive_{}", uuid::Uuid::new_v4()),
            node_type: "mode_preventive".to_string(),
            success: true,
            output: NodeOutput::Clarification { questions: vec!["Apa klausul atau tindakan yang ingin diamankan?".to_string()] },
            confidence: synthesis.certainty,
            warnings: Vec::new(),
            metadata: serde_json::json!({"interpretation": interpretation, "risk_matrix": risk_matrix, "checklist": checklist, "certainty": synthesis.certainty}),
        })
    }

    fn node_type(&self) -> &'static str {
        "mode_preventive"
    }
}
