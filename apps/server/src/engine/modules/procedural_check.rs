use tracing::info;
use crate::engine::context::ExecutionContext;
use crate::engine::EngineError;

pub struct ProceduralChecker;

impl ProceduralChecker {
    pub fn new() -> Self {
        Self
    }

    pub fn check(&self, context: &ExecutionContext) -> Result<String, EngineError> {
        info!(case_id=%context.case_id, "performing procedural and formal law checks");

        let mut report = String::from("PEMERIKSAAN PROSEDURAL & FORMIL (MANDATORY)\n============================================\n\n");

        report.push_str("1. DALUWARSA (Statute of Limitations)\n");
        report.push_str("   Status: Perlu verifikasi tanggal kejadian vs batas waktu gugat\n");
        report.push_str("   Rekomendasi: Hitung selisih waktu kejadian dengan hari ini\n\n");

        report.push_str("2. KEWENANGAN PENGADILAN\n");
        report.push_str("   Absolut: Pengadilan Negeri (Sengketa Perdata)\n");
        report.push_str("   Relatif: Periksa domisili tergugat dan tempat peristiwa\n");
        report.push_str("   Rekomendasi: Verifikasi kewenangan forum yang dipilih\n\n");

        report.push_str("3. NE BIS IN IDEM\n");
        report.push_str("   Status: Tidak ada informasi perkara sebelumnya\n");
        report.push_str("   Rekomendasi: Cek apakah perkara yang sama pernah diputus\n\n");

        report.push_str("4. ERROR IN PERSONA\n");
        report.push_str("   Status: Perlu verifikasi identitas pihak yang digugat\n");
        report.push_str("   Rekomendasi: Pastikan pihak yang digugat adalah pihak yang tepat\n\n");

        report.push_str("5. KLAUSULA PENYELESAIAN SENGKETA\n");
        report.push_str("   Status: Perlu review dokumen perjanjian\n");
        report.push_str("   Rekomendasi: Cek keberadaan klausul arbitrase atau mediasi wajib\n\n");

        report.push_str("Kesimpulan:\n");
        report.push_str("  Pemeriksaan prosedural dan formil adalah langkah MANDATORY sebelum litigasi.\n");
        report.push_str("  Pastikan semua kelengkapan formil terpenuhi untuk menghindari gugatan ditolak.\n");

        Ok(report)
    }
}
