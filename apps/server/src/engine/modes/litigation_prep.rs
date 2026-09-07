use tracing::info;
use crate::engine::context::ExecutionContext;
use crate::engine::nodes::{NodeExecutor, NodeOutput, NodeResult};
use crate::engine::layers::base::{LegalLayer, LayerOutput};
use crate::engine::EngineError;

pub struct LitigationPrepMode;

impl LitigationPrepMode {
    pub fn new() -> Self {
        Self
    }

    fn build_case_theory(&self, context: &ExecutionContext) -> String {
        let conclusions: Vec<&str> = context.case_graph.conclusions.iter().map(|c| c.content.as_str()).collect();
        let facts: Vec<&str> = context.case_graph.facts.iter().map(|f| f.content.as_str()).collect();
        let rules: Vec<&str> = context.case_graph.rules.iter().map(|r| r.content.as_str()).collect();

        format!(
            "TEORI KASUS\n===========\n\nFakta Utama:\n{}\n\nDasar Hukum:\n{}\n\nKesimpulan:\n{}\n\nStrategi Pembuktian:\n1. Kumpulkan dokumen pendukung untuk setiap fakta\n2. Siapkan saksi yang kompeten\n3. Siapkan bukti objektif (surat, foto, rekaman)\n4. Antisipasi pembelaan lawan\n5. Siapkan argumen untuk setiap isu hukum yang teridentifikasi",
            facts.join("\n- ").chars().take(500).collect::<String>(),
            rules.join("\n- ").chars().take(500).collect::<String>(),
            conclusions.join("\n- ").chars().take(500).collect::<String>()
        )
    }

    fn build_evidence_chart(&self, context: &ExecutionContext) -> Vec<(String, String, String, String)> {
        let mut chart = Vec::new();

        for fact in &context.case_graph.facts {
            chart.push((
                fact.content.clone(),
                "Bukti pendukung diperlukan".to_string(),
                "Belum terverifikasi".to_string(),
                "Tingkat kepentingan: Sedang".to_string(),
            ));
        }

        for rule in &context.case_graph.rules {
            chart.push((
                format!("Peraturan: {}", rule.content),
                "Teks lengkap peraturan".to_string(),
                "Terverifikasi dari Knowledge Base".to_string(),
                "Tingkat kepentingan: Tinggi".to_string(),
            ));
        }

        if chart.is_empty() {
            chart.push((
                "Umum".to_string(),
                "Perlu identifikasi bukti".to_string(),
                "Belum ada".to_string(),
                "Tingkat kepentingan: Tinggi".to_string(),
            ));
        }

        chart
    }

    async fn run_litigation_layers(&self, context: &mut ExecutionContext) -> Result<Vec<LayerOutput>, EngineError> {
        let mut layers = Vec::new();

        let grammatical = crate::engine::engine::layers::grammatical::GrammaticalInterpreter::new();
        layers.push(grammatical.analyze(context).await?);

        let systematic = crate::engine::engine::layers::systematic::SystematicInterpreter::new();
        layers.push(systematic.analyze(context).await?);

        let comparative = crate::engine::engine::layers::comparative::ComparativeInterpreter::new();
        layers.push(comparative.analyze(context).await?);

        let critical = crate::engine::engine::layers::critical::CriticalInterpreter::new();
        layers.push(critical.analyze(context).await?);

        layers
    }
}

#[async_trait::async_trait]
impl NodeExecutor for LitigationPrepMode {
    async fn execute(&self, context: &mut ExecutionContext) -> Result<NodeResult, EngineError> {
        context.check_cancelled()?;
        info!(case_id=%context.case_id, mode="litigation_prep", "executing litigation preparation mode");

        let layers = Self::run_litigation_layers(self, context).await?;
        let case_theory = Self::build_case_theory(self, context);
        let evidence_chart = Self::build_evidence_chart(self, context);

        let synthesis = crate::engine::engine::layers::synthesis::SynthesisEngine::new()
            .synthesize(&layers, context)?;

        let interpretation = format!(
            "MODE PERSIAPAN LITIGASI — Teori Kasus & Struktur Pembuktian\n\
             ==========================================================\n\n{}\n\n\
             TABEL STRUKTUR PEMBUKTIAN:\n{}\n\n\
             Tingkat kepastian: {:.0}%\n\n\
             Rekomendasi: Pastikan semua bukti terverifikasi sebelum mengajukan ke pengadilan.",
            case_theory,
            evidence_chart.iter().enumerate().map(|(i, (f, e, s, l))| format!("{}. Fakta: {}\n   Bukti: {}\n   Status: {}\n   Level: {}", i + 1, f, e, s, l)).collect::<Vec<_>>().join("\n\n"),
            synthesis.certainty * 100.0
        );

        Ok(NodeResult {
            node_id: format!("litigation_prep_{}", uuid::Uuid::new_v4()),
            node_type: "mode_litigation_prep".to_string(),
            success: true,
            output: NodeOutput::Clarification { questions: vec!["Forum atau prosedur apa yang sudah dipilih?".to_string()] },
            confidence: synthesis.certainty,
            warnings: Vec::new(),
            metadata: serde_json::json!({"interpretation": interpretation, "case_theory": case_theory, "evidence_chart": evidence_chart, "certainty": synthesis.certainty}),
        })
    }

    fn node_type(&self) -> &'static str {
        "mode_litigation_prep"
    }
}
