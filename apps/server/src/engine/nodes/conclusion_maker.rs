use tracing::info;
use crate::engine::context::ExecutionContext;
use crate::engine::nodes::{NodeExecutor, NodeOutput, NodeResult, ConclusionEntry};
use crate::engine::EngineError;

pub struct ConclusionMaker;

impl ConclusionMaker {
    pub fn new() -> Self {
        Self
    }

    fn draw_conclusions(&self, context: &ExecutionContext) -> Vec<ConclusionEntry> {
        let mut conclusions = Vec::new();
        let arguments: Vec<&str> = context.case_graph.arguments.iter().map(|a| a.content.as_str()).collect();
        let risks: Vec<&str> = context.case_graph.risks.iter().map(|r| r.content.as_str()).collect();

        if !arguments.is_empty() {
            let avg_certainty = arguments.len() as f32 * 0.1;
            conclusions.push(ConclusionEntry {
                id: format!("conc_{}", uuid::Uuid::new_v4()),
                statement: "Berdasarkan analisis yang telah dilakukan, posisi hukum pengguna memiliki dasar yang cukup kuat untuk dilanjutkan ke tahap berikutnya.".to_string(),
                supporting_arguments: arguments.iter().take(3).cloned().map(String::from).collect(),
                conditions: vec![
                    "Fakta harus terverifikasi dengan dokumen pendukung".to_string(),
                    "Bukti harus cukup meyakinkan".to_string(),
                ],
                alternative_conclusions: vec![
                    "Hasil dapat berbeda jika fakta baru ditemukan".to_string(),
                    "Interpretasi hukum dapat berbeda antar hakim".to_string(),
                ],
                certainty: (0.5 + avg_certainty).min(0.95),
            });
        }

        if !risks.is_empty() {
            conclusions.push(ConclusionEntry {
                id: format!("conc_{}", uuid::Uuid::new_v4()),
                statement: format!("Ada {} risiko utama yang perlu diantisipasi dan dimitigasi sebelum melanjutkan.", risks.len()),
                supporting_arguments: risks.iter().take(2).cloned().map(String::from).collect(),
                conditions: vec!["Mitigasi risiko harus dijalankan sebelum tindakan hukum selanjutnya".to_string()],
                alternative_conclusions: vec!["Risiko dapat berkurang jika bukti tambahan ditemukan".to_string()],
                certainty: 0.7,
            });
        }

        if conclusions.is_empty() {
            conclusions.push(ConclusionEntry {
                id: format!("conc_{}", uuid::Uuid::new_v4()),
                statement: "Belum cukup informasi untuk menarik kesimpulan hukum yang definitif. Diperlukan klarifikasi lebih lanjut dan pengumpulan bukti tambahan.".to_string(),
                supporting_arguments: Vec::new(),
                conditions: vec![
                    "Perlu klarifikasi fakta lebih lanjut".to_string(),
                    "Perlu bukti pendukung".to_string(),
                    "Perlu identifikasi peraturan yang relevan".to_string(),
                ],
                alternative_conclusions: Vec::new(),
                certainty: 0.3,
            });
        }

        conclusions
    }
}

#[async_trait::async_trait]
impl NodeExecutor for ConclusionMaker {
    async fn execute(&self, context: &mut ExecutionContext) -> Result<NodeResult, EngineError> {
        context.check_cancelled()?;
        info!(case_id=%context.case_id, "drawing conclusions");

        let conclusions = Self::draw_conclusions(self, context);

        for conclusion in &conclusions {
            context.case_graph.conclusions.push(crate::engine::context::GraphNode {
                id: conclusion.id.clone(),
                node_type: "conclusion".to_string(),
                content: conclusion.statement.clone(),
                metadata: serde_json::json!({"certainty": conclusion.certainty, "conditions": conclusion.conditions}),
                certainty: conclusion.certainty,
            });
        }

        Ok(NodeResult {
            node_id: format!("conclusion_{}", uuid::Uuid::new_v4()),
            node_type: "conclusion_maker".to_string(),
            success: true,
            output: NodeOutput::Conclusions { conclusions: conclusions.clone() },
            confidence: 0.6,
            warnings: Vec::new(),
            metadata: serde_json::json!({"total_conclusions": conclusions.len()}),
        })
    }

    fn node_type(&self) -> &'static str {
        "conclusion_maker"
    }
}
