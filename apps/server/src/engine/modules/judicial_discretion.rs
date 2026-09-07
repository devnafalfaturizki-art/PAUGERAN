use tracing::info;
use crate::engine::context::ExecutionContext;
use crate::engine::EngineError;

pub struct JudicialDiscretionModeler;

impl JudicialDiscretionModeler {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze(&self, context: &ExecutionContext) -> Result<String, EngineError> {
        info!(case_id=%context.case_id, "modeling judicial discretion");

        let facts: Vec<&str> = context.case_graph.facts.iter().map(|f| f.content.as_str()).collect();
        let risks: Vec<&str> = context.case_graph.risks.iter().map(|r| r.content.as_str()).collect();

        let mut analysis = String::from("ANALISIS KELELUASAAN HAKIM (VRIJE BEWIJSWAARDERING)\n=====================================================\n\n");
        analysis.push_str("Pertimbangan Hakim:\n");
        analysis.push_str("  - Alat bukti surat > Saksi > Persangkaan > Pengakuan > Sumpah\n");
        analysis.push_str("  - Kredibilitas saksi dan kepentingan saksi dievaluasi\n");
        analysis.push_str("  - Kebebasan hakim dalam menilai bukti (Pasal 1865 HIR / 163 RBg)\n\n");

        if !facts.is_empty() {
            analysis.push_str("Bukti yang Dikeluarkan:\n");
            for fact in facts.iter().take(5) {
                analysis.push_str(&format!("  - {}\n", fact.chars().take(100).collect::<String>()));
            }
        }

        if !risks.is_empty() {
            analysis.push_str("\nRisiko Penilaian Hakim:\n");
            for risk in risks.iter().take(3) {
                analysis.push_str(&format!("  - {}\n", risk.chars().take(100).collect::<String>()));
            }
        }

        analysis.push_str("\nRekomendasi:\n");
        analysis.push_str("  - Siapkan saksi yang kredibel dan objektif\n");
        analysis.push_str("  - Perkuat bukti dokumen untuk mengurangi ketergantungan pada kesaksian lisan\n");
        analysis.push_str("  - Antisipasi variabel penilaian hakim dalam strategi litigasi\n");

        Ok(analysis)
    }
}
