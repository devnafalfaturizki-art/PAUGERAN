use tracing::info;
use crate::engine::context::ExecutionContext;
use crate::engine::EngineError;

pub struct MultiJurisdictionalAnalyzer;

impl MultiJurisdictionalAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze(&self, context: &ExecutionContext) -> Result<String, EngineError> {
        info!(case_id=%context.case_id, "performing multi-jurisdictional analysis");

        let facts: Vec<&str> = context.case_graph.facts.iter().map(|f| f.content.as_str()).collect();
        let combined = facts.join(" ").to_lowercase();

        let mut report = String::from("ANALISIS MULTI-YURISDIKSI\n==========================\n\n");

        if combined.contains("asing") || combined.contains("luar negeri") || combined.contains("internasional") || combined.contains("export") || combined.contains("import") {
            report.push_str("Unsur Internasional Terdeteksi:\n\n");
            report.push_str("1. PILIHAN HUKUM (Choice of Law)\n");
            report.push_str("   - Identifikasi hukum mana yang berlaku untuk kontrak\n");
            report.push_str("   - Periksa klausul choice of law dalam perjanjian\n\n");

            report.push_str("2. YURISDIKSI YANG BERWENANG\n");
            report.push_str("   - Tentukan forum yang berwenang mengadili sengketa\n");
            report.push_str("   - Periksa klausul choice of forum\n\n");

            report.push_str("3. PENGAKUAN & EKSEKUSI PUTUSAN ASING\n");
            report.push_str("   - Berdasarkan asas timbal balik\n");
            report.push_str("   - Verifikasi apakah putusan asing dapat dieksekusi di Indonesia\n");
        } else {
            report.push_str("Tidak ada indikasi kasus multi-yurisdiksi dalam konteks ini.\n");
            report.push_str("Jika kasus melibatkan unsur asing, berikan detail untuk analisis lebih lanjut.\n");
        }

        Ok(report)
    }
}
