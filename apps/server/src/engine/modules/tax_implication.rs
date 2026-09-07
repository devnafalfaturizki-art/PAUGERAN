use tracing::info;
use crate::engine::context::ExecutionContext;
use crate::engine::EngineError;

pub struct TaxImplicationAnalyzer;

impl TaxImplicationAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze(&self, context: &ExecutionContext) -> Result<String, EngineError> {
        info!(case_id=%context.case_id, "analyzing tax implications");

        let facts: Vec<&str> = context.case_graph.facts.iter().map(|f| f.content.as_str()).collect();
        let combined = facts.join(" ").to_lowercase();

        let mut report = String::from("ANALISIS IMPLIKASI PAJAK\n========================\n\n");

        if combined.contains("jual") || combined.contains("beli") || combined.contains("transaksi") || combined.contains("bayar") {
            report.push_str("Transaksi Hukum Terdeteksi — Potensi Implikasi Pajak:\n\n");
            report.push_str("1. PAJAK PENGHASILAN (PPh)\n");
            report.push_str("   - PPh 21: Untuk transaksi involving karyawan/pegawai\n");
            report.push_str("   - PPh 23/26: Untuk jasa dan royalti\n");
            report.push_str("   - PPh 25: Untuk pembayaran cicilan\n\n");

            report.push_str("2. PAJAK PERTAMBAHAN NILAI (PPN)\n");
            report.push_str("   - PPN 11% untuk penyerahan BKP/JKP\n");
            report.push_str("   - PPN ekspor/impor\n\n");

            report.push_str("3. BEA PEROLEHAN HAK TANAH BANGUNAN (BPHTB)\n");
            report.push_str("   - Jika transaksi melibatkan tanah/bangunan\n\n");

            report.push_str("Rekomendasi:\n");
            report.push_str("  - Konsultasikan dengan konsultan pajak untuk perhitungan yang akurat\n");
            report.push_str("  - Pertimbangkan implikasi pajak dalam struktur transaksi\n");
            report.push_str("  - Dokumentasikan dasar perhitungan pajak\n");
        } else {
            report.push_str("Tidak ada indikasi transaksi dengan implikasi pajak yang signifikan.\n");
            report.push_str("Namun, setiap transaksi hukum sebaiknya dievaluasi dari segi pajak.\n");
        }

        Ok(report)
    }
}
