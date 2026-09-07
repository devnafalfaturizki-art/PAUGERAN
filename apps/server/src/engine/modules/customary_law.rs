use tracing::info;
use crate::engine::context::ExecutionContext;
use crate::engine::EngineError;

pub struct CustomaryLawIntegrator;

impl CustomaryLawIntegrator {
    pub fn new() -> Self {
        Self
    }

    pub fn integrate(&self, context: &ExecutionContext) -> Result<String, EngineError> {
        info!(case_id=%context.case_id, "integrating unwritten and customary law");

        let facts: Vec<&str> = context.case_graph.facts.iter().map(|f| f.content.as_str()).collect();
        let combined = facts.join(" ").to_lowercase();

        let mut integration = String::from("INTEGRASI HUKUM ADAT & TIDAK TERTULIS\n=======================================\n\n");

        if combined.contains("bali") || combined.contains("adat") || combined.contains("suku") {
            integration.push_str("Konteks Adat Terdeteksi:\n");
            integration.push_str("  - Hukum adat setempat perlu dipertimbangkan sebagai living law\n");
            integration.push_str("  - Pasal 18B UUD 1945 mengakui keberadaan hukum adat\n");
            integration.push_str("  - Hukum adat berlaku selaku sumber hukum yang hidup di masyarakat\n\n");
            integration.push_str("Catatan:\n");
            integration.push_str("  - Hukum negara (UU) sebagai lex superior\n");
            integration.push_str("  - Namun hukum adat tetap diakui sebagai living law\n");
            integration.push_str("  - Pertimbangkan konsultasi dengan tetua adat atau ahli hukum adat\n");
        } else if combined.contains("waris") || combined.contains("warisan") {
            integration.push_str("Konteks Waris:\n");
            integration.push_str("  - Hukum waris adat mungkin berlaku selain hukum negara\n");
            integration.push_str("  - Sistem waris adat (misal: purusa/pradana di Bali) perlu diverifikasi\n");
            integration.push_str("  - Konsultasikan dengan ahli waris dan tetua adat\n");
        } else if combined.contains("tanah") || combined.contains("ulayat") {
            integration.push_str("Konteks Tanah Adat:\n");
            integration.push_str("  - Tanah ulayat diatur oleh hukum adat setempat\n");
            integration.push_str("  - Hak ulayat diakui oleh UU No. 5 Tahun 1960 (UUPA)\n");
            integration.push_str("  - Perhatikan asas- asas hukum adat dalam peralihan tanah\n");
        } else {
            integration.push_str("Tidak ada indikasi kuat keterlibatan hukum adat dalam kasus ini.\n");
            integration.push_str("Namun, jika kasus melibatkan aspek agraria, waris, atau bisnis keluarga,\n");
            integration.push_str("pertimbangkan untuk melakukan integrasi hukum adat.\n");
        }

        Ok(integration)
    }
}
