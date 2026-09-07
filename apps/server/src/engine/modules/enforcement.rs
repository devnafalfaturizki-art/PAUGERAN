use tracing::info;
use crate::engine::context::ExecutionContext;
use crate::engine::EngineError;

pub struct EnforcementStrategist;

impl EnforcementStrategist {
    pub fn new() -> Self {
        Self
    }

    pub fn strategize(&self, context: &ExecutionContext) -> Result<String, EngineError> {
        info!(case_id=%context.case_id, "developing enforcement strategy");

        let risks: Vec<&str> = context.case_graph.risks.iter().map(|r| r.content.as_str()).collect();
        let conclusions: Vec<&str> = context.case_graph.conclusions.iter().map(|c| c.content.as_str()).collect();

        let mut report = String::from("STRATEGI EKSEKUSI PUTUSAN\n=========================\n\n");

        report.push_str("1. PRA-PENYELESAIAN\n");
        report.push_str("   - Pastikan putusan sudah berkekuatan hukum tetap (inkracht)\n");
        report.push_str("   - Beri kesempatan pelunasan sukarela sebelum eksekusi\n\n");

        report.push_str("2. EKSEKUSI MELALUI PENGHUKUMAN\n");
        report.push_str("   - Sita jaminan dan harta kekayaan tergugat\n");
        report.push_str("   - Evaluasi aset yang dapat dieksekusi\n");
        report.push_str("   - Koordinasi dengan jaksa penjagaan kekayaan\n\n");

        if !risks.is_empty() {
            report.push_str("3. HAMBATAN EKSEKUSI YANG DITEMUKAN:\n");
            for risk in risks.iter().take(3) {
                report.push_str(&format!("   - {}\n", risk.chars().take(100).collect::<String>()));
            }
            report.push_str("\n");
        }

        if !conclusions.is_empty() {
            report.push_str("4. DASAR KESIMPULAN UNTUK EKSEKUSI:\n");
            for conclusion in conclusions.iter().take(3) {
                report.push_str(&format!("   - {}\n", conclusion.chars().take(100).collect::<String>()));
            }
            report.push_str("\n");
        }

        report.push_str("Rekomendasi:\n");
        report.push_str("  - Siapkan rencana eksekusi yang realistis dan dapat dilaksanakan\n");
        report.push_str("  - Pertimbangkan alternatif penyelesaian jika eksekusi tidak mungkin\n");
        report.push_str("  - Dokumentasikan semua langkah eksekusi untuk audit trail\n");

        Ok(report)
    }
}
