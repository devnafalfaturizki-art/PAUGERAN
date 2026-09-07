use tracing::info;
use crate::engine::context::ExecutionContext;
use crate::engine::EngineError;

pub struct CausationAnalyzer;

impl CausationAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze(&self, context: &ExecutionContext) -> Result<String, EngineError> {
        info!(case_id=%context.case_id, "analyzing causation and remoteness");

        let facts: Vec<&str> = context.case_graph.facts.iter().map(|f| f.content.as_str()).collect();
        let issues: Vec<&str> = context.case_graph.issues.iter().map(|i| i.content.as_str()).collect();

        let mut analysis = String::from("ANALISIS KAUSALITAS & REMOTENESS\n=================================\n\n");

        analysis.push_str("Rantai Kausalitas:\n");
        analysis.push_str("  Perbuatan -> Akibat Langsung (Proximate Cause) -> Akibat Lanjutan (Remote Cause)\n\n");

        analysis.push_str("Uji 'But-For' (Sine Qua Non):\n");
        analysis.push_str("  Apakah kerugian terjadi jika tidak ada perbuatan tersebut?\n\n");

        analysis.push_str("Uji Foreseeability:\n");
        analysis.push_str("  Apakah kerugian dapat diduga secara wajar pada saat perjanjian dibuat?\n\n");

        if !facts.is_empty() {
            analysis.push_str("Fakta yang Dianalisis:\n");
            for fact in facts.iter().take(5) {
                analysis.push_str(&format!("  - {}\n", fact.chars().take(100).collect::<String>()));
            }
        }

        if !issues.is_empty() {
            analysis.push_str("\nIsu Hukum:\n");
            for issue in issues.iter().take(3) {
                analysis.push_str(&format!("  - {}\n", issue.chars().take(100).collect::<String>()));
            }
        }

        analysis.push_str("\nRekomendasi:\n");
        analysis.push_str("  - Fokus gugatan pada kerugian langsung (proximate cause)\n");
        analysis.push_str("  - Hindari gugatan kerugian yang terlalu jauh (remote cause) kecuali dapat dibuktikan\n");

        Ok(analysis)
    }
}
