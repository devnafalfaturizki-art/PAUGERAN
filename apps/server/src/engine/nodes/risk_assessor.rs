use tracing::info;
use crate::engine::context::ExecutionContext;
use crate::engine::nodes::{NodeExecutor, NodeOutput, NodeResult, RiskEntry, RiskImpact};
use crate::engine::EngineError;

pub struct RiskAssessor;

impl RiskAssessor {
    pub fn new() -> Self {
        Self
    }

    fn assess_risks(&self, context: &ExecutionContext) -> Vec<RiskEntry> {
        let mut risks = Vec::new();
        let facts: Vec<&str> = context.case_graph.facts.iter().map(|f| f.content.as_str()).collect();
        let combined = facts.join(" ").to_lowercase();

        if combined.contains("tanah") || combined.contains("properti") {
            risks.push(RiskEntry {
                id: format!("risk_{}", uuid::Uuid::new_v4()),
                description: "Risiko sengketa kepemilikan tanah atau status sertifikat yang belum jelas".to_string(),
                probability: 0.6,
                impact: RiskImpact::High,
                mitigation: "Lakukan pengecekan sertifikat di BPN, periksa riwayat peralihan, dan konsultasi dengan notaris".to_string(),
                residual_risk: 0.3,
            });
        }

        if combined.contains("karyawan") || combined.contains("phk") {
            risks.push(RiskEntry {
                id: format!("risk_{}", uuid::Uuid::new_v4()),
                description: "Risiko gugatan PHK dari mantan karyawan atau sengketa hubungan kerja".to_string(),
                probability: 0.5,
                impact: RiskImpact::High,
                mitigation: "Pastikan prosedur PHK sesuai UU Ketenagakerjaan, dokumentasikan alasan PHK, dan lakukan mediasi sebelum PHK".to_string(),
                residual_risk: 0.25,
            });
        }

        if combined.contains("kontrak") || combined.contains("perjanjian") {
            risks.push(RiskEntry {
                id: format!("risk_{}", uuid::Uuid::new_v4()),
                description: "Risiko wanprestasi atau perjanjian yang tidak dapat dilaksanakan".to_string(),
                probability: 0.4,
                impact: RiskImpact::Medium,
                mitigation: "Sertakan klausul sanksi, mekanisme penyelesaian sengketa, dan force majeure dalam perjanjian".to_string(),
                residual_risk: 0.2,
            });
        }

        if combined.contains("warisan") {
            risks.push(RiskEntry {
                id: format!("risk_{}", uuid::Uuid::new_v4()),
                description: "Risiko sengketa waris dengan ahli waris lain atau perbedaan penafsiran wasiat".to_string(),
                probability: 0.55,
                impact: RiskImpact::High,
                mitigation: "Pastikan surat wasiat disusun dengan notaris, lakukan musyawarah keluarga, dan dokumentasikan pembagian harta".to_string(),
                residual_risk: 0.3,
            });
        }

        if risks.is_empty() {
            risks.push(RiskEntry {
                id: format!("risk_{}", uuid::Uuid::new_v4()),
                description: "Risiko umum terkait ketidakpastian fakta dan hukum".to_string(),
                probability: 0.3,
                impact: RiskImpact::Medium,
                mitigation: "Kumpulkan lebih banyak bukti, konsultasikan dengan ahli hukum, dan pertimbangkan alternatif penyelesaian".to_string(),
                residual_risk: 0.15,
            });
        }

        risks
    }
}

#[async_trait::async_trait]
impl NodeExecutor for RiskAssessor {
    async fn execute(&self, context: &mut ExecutionContext) -> Result<NodeResult, EngineError> {
        context.check_cancelled()?;
        info!(case_id=%context.case_id, "assessing risks");

        let risks = Self::assess_risks(self, context);

        let total_risks = risks.len();

        for risk in &risks {
            context.case_graph.risks.push(crate::engine::context::GraphNode {
                id: risk.id.clone(),
                node_type: "risk".to_string(),
                content: risk.description.clone(),
                metadata: serde_json::json!({"probability": risk.probability, "impact": format!("{:?}", risk.impact).to_lowercase(), "residual_risk": risk.residual_risk}),
                certainty: 1.0 - risk.probability,
            });
        }

        Ok(NodeResult {
            node_id: format!("risk_assess_{}", uuid::Uuid::new_v4()),
            node_type: "risk_assessor".to_string(),
            success: true,
            output: NodeOutput::Risks { risks: risks.clone() },
            confidence: 0.6,
            warnings: Vec::new(),
            metadata: serde_json::json!({"total_risks": total_risks}),
        })
    }

    fn node_type(&self) -> &'static str {
        "risk_assessor"
    }
}
