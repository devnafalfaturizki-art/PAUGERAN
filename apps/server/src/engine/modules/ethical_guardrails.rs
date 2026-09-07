use tracing::info;
use crate::engine::context::ExecutionContext;
use crate::engine::EngineError;

pub struct EthicalGuardrails;

impl EthicalGuardrails {
    pub fn new() -> Self {
        Self
    }

    fn check_ethical_violations(&self, context: &ExecutionContext) -> Vec<String> {
        let mut violations = Vec::new();

        for arg in &context.case_graph.arguments {
            let content_lower = arg.content.to_lowercase();
            if content_lower.contains("memalsukan") || content_lower.contains("menyuap") || content_lower.contains("mengada-ada") {
                violations.push(format!("Potensi pelanggaran etika dalam argumen: {}", arg.content));
            }
        }

        for risk in &context.case_graph.risks {
            let content_lower = risk.content.to_lowercase();
            if content_lower.contains("menipu") || content_lower.contains("mengelabui") {
                violations.push(format!("Potensi pelanggaran etika dalam risiko: {}", risk.content));
            }
        }

        if violations.is_empty() {
            violations.push("Tidak ada pelanggaran etika yang terdeteksi.".to_string());
        }

        violations
    }

    pub fn validate(&self, context: &ExecutionContext) -> Result<String, EngineError> {
        info!(case_id=%context.case_id, "checking ethical guardrails");

        let violations = Self::check_ethical_violations(self, context);

        let mut report = String::from("PERINGATAN ETIKA PROFESI (ETHICAL GUARDRAIL)\n=============================================\n\n");

        for (i, violation) in violations.iter().enumerate() {
            if violation == "Tidak ada pelanggaran etika yang terdeteksi." {
                report.push_str(&format!("{}. {}\n", i + 1, violation));
            } else {
                report.push_str(&format!("\u{26A0} {} \n", i + 1, violation));
            }
        }

        report.push_str("\nPrinsip Etika yang Diterapkan:\n");
        report.push_str("  - Kode Etik Advokat Indonesia (PERADI)\n");
        report.push_str("  - Larangan taktik frivolous, vexatious, atau melanggar etika\n");
        report.push_str("  - Kewajiban menjaga rahasia klien\n");
        report.push_str("  - Larangan konflik kepentingan\n");

        Ok(report)
    }
}
