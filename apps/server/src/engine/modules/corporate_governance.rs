use tracing::info;
use crate::engine::context::ExecutionContext;
use crate::engine::EngineError;

pub struct CorporateGovernanceChecker;

impl CorporateGovernanceChecker {
    pub fn new() -> Self {
        Self
    }

    pub fn check(&self, context: &ExecutionContext) -> Result<String, EngineError> {
        info!(case_id=%context.case_id, "performing corporate governance check");

        let facts: Vec<&str> = context.case_graph.facts.iter().map(|f| f.content.as_str()).collect();
        let combined = facts.join(" ").to_lowercase();

        let mut report = String::from("Pemeriksaan CORPORATE GOVERNANCE & REGULATORY COMPLIANCE\n=========================================================\n\n");

        if combined.contains("pt") || combined.contains("perusahaan") || combined.contains("direktur") || combined.contains("komisaris") {
            report.push_str("Konteks Korporasi Terdeteksi:\n\n");
            report.push_str("1. TATA KELOLA PERUSAHAAN\n");
            report.push_str("   - Periksa kepatuhan terhadap RUPS\n");
            report.push_str("   - Verifikasi kewenangan direksi/komisaris (Pasal 97-98 UU PT)\n");
            report.push_str("   - Evaluasi conflict of interest\n\n");

            report.push_str("2. KEPATUHAN REGULASI SEKTORAL\n");
            report.push_str("   - OJK: Jika perusahaan berjalan di bidang jasa keuangan\n");
            report.push_str("   - KPPU: Jika ada indikasi monopoli atau kartel\n");
            report.push_str("   - Kemenkumham: Untuk pencatangan perusahaan\n\n");

            report.push_str("Rekomendasi:\n");
            report.push_str("  - Review dokumen anggaran dasar dan sisa ketentuan\n");
            report.push_str("  - Verifikasi keputusan direksi/komisaris sesuai wewenang\n");
            report.push_str("  - Pastikan kepatuhan terhadap laporan keuangan dan kewajiban disclosure\n");
        } else {
            report.push_str("Tidak ada indikasi kasus korporasi dalam konteks ini.\n");
            report.push_str("Jika kasus melibatkan korporasi, berikan detail untuk analisis lebih lanjut.\n");
        }

        Ok(report)
    }
}
