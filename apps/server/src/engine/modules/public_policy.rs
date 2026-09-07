use tracing::info;
use crate::engine::context::ExecutionContext;
use crate::engine::EngineError;

pub struct PublicPolicyFilter;

impl PublicPolicyFilter {
    pub fn new() -> Self {
        Self
    }

    pub fn filter(&self, context: &ExecutionContext) -> Result<String, EngineError> {
        info!(case_id=%context.case_id, "applying public policy filter");

        let rules: Vec<&str> = context.case_graph.rules.iter().map(|r| r.content.as_str()).collect();
        let arguments: Vec<&str> = context.case_graph.arguments.iter().map(|a| a.content.as_str()).collect();

        let mut report = String::from("PUBLIC POLICY FILTER\n=====================\n\n");

        report.push_str("1. KONFLIK DENGAN KEBIJAKAN PUBLIK\n");
        report.push_str("   - Evaluasi apakah klausul atau tindakan yang dianalisis bertentangan dengan kebijakan publik\n");
        report.push_str("   - Contoh: Klausul yang melanggar hukum consumer protection\n\n");

        report.push_str("2. BATAL DEMI HUKUM (NUL LEX SINE LEGE)\n");
        report.push_str("   - Jika perjanjian atau tindakan bertentangan dengan hukum yang berlaku\n");
        report.push_str("   - maka dapat batal demi hukum\n\n");

        if !rules.is_empty() {
            report.push_str("\n3. PERATURAN YANG DIIKUTI:\n");
            for rule in rules.iter().take(5) {
                report.push_str(&format!("   - {}\n", rule.chars().take(100).collect::<String>()));
            }
        }

        if !arguments.is_empty() {
            report.push_str("\n4. ARGUMEN YANG DIIKUTI:\n");
            for arg in arguments.iter().take(3) {
                report.push_str(&format!("   - {}\n", arg.chars().take(100).collect::<String>()));
            }
        }

        report.push_str("\nRekomendasi:\n");
        report.push_str("  - Hindari strategi yang bertentangan dengan kebijakan publik\n");
        report.push_str("  - Konsultasikan dengan ahli hukum jika ada keraguan\n");

        Ok(report)
    }
}
