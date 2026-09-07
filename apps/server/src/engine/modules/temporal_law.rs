use tracing::info;
use crate::engine::context::ExecutionContext;
use crate::engine::EngineError;

pub struct TemporalLawApplier;

impl TemporalLawApplier {
    pub fn new() -> Self {
        Self
    }

    pub fn apply(&self, context: &ExecutionContext) -> Result<String, EngineError> {
        info!(case_id=%context.case_id, "applying temporal law principles");

        let rules: Vec<&str> = context.case_graph.rules.iter().map(|r| r.content.as_str()).collect();
        let facts: Vec<&str> = context.case_graph.facts.iter().map(|f| f.content.as_str()).collect();

        let mut application = String::from("PENERAPAN HUKUM BERDASARKAN WAKTU (LEX TEMPORIS)\n==================================================\n\n");
        application.push_str("Tanggal Kejadian: Perlu identifikasi tanggal kejadian kritis dari fakta.\n\n");
        application.push_str("Peraturan yang Berlaku:\n");

        for rule in rules.iter().take(5) {
            application.push_str(&format!("  - {}\n", rule.chars().take(100).collect::<String>()));
        }

        application.push_str("\nCatatan Penting:\n");
        application.push_str("  - Hukum yang berlaku pada saat peristiwa terjadi adalah yang diterapkan (lex temporis).\n");
        application.push_str("  - Hukum baru tidak berlaku surut kecuali ada ketentuan eksplisit.\n");
        application.push_str("  - Aturan peralihan perlu diverifikasi untuk setiap perubahan peraturan.\n");

        Ok(application)
    }
}
