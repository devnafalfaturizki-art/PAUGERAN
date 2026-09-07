use tracing::info;
use crate::engine::context::ExecutionContext;
use crate::engine::EngineError;

pub struct NormConflictResolver;

impl NormConflictResolver {
    pub fn new() -> Self {
        Self
    }

    fn detect_conflicts(&self, context: &ExecutionContext) -> Vec<(String, String, String, String)> {
        let mut conflicts = Vec::new();
        let rules: Vec<&str> = context.case_graph.rules.iter().map(|r| r.content.as_str()).collect();

        if rules.len() >= 2 {
            for i in 0..rules.len().min(5) {
                for j in (i + 1)..rules.len().min(5) {
                    conflicts.push((
                        rules[i].to_string(),
                        rules[j].to_string(),
                        format!("lex_superior_{}", i),
                        "Peraturan yang lebih tinggi mengesampingkan yang lebih rendah".to_string(),
                    ));
                }
            }
        }

        if conflicts.is_empty() {
            conflicts.push((
                "Tidak ada konflik norma yang terdeteksi".to_string(),
                "".to_string(),
                "tidak_ada_konflik".to_string(),
                "Semua peraturan yang berlaku konsisten".to_string(),
            ));
        }

        conflicts
    }

    pub fn resolve(&self, context: &ExecutionContext) -> Result<String, EngineError> {
        info!(case_id=%context.case_id, "resolving norm conflicts");

        let conflicts = self.detect_conflicts(context);

        if conflicts.len() == 1 && conflicts[0].2 == "tidak_ada_konflik" {
            return Ok("Tidak ada konflik norma yang terdeteksi.".to_string());
        }

        let mut resolution = String::from("RESOLUSI KONFLIK NORMA\n=======================\n\n");
        for (i, (a, b, principle, explanation)) in conflicts.iter().enumerate().take(5) {
            resolution.push_str(&format!(
                "{}. Konflik antara:\n   A: {}\n   B: {}\n   Prinsip: {}\n   Resolusi: {}\n\n",
                i + 1,
                a.chars().take(100).collect::<String>(),
                b.chars().take(100).collect::<String>(),
                principle,
                explanation
            ));
        }

        Ok(resolution)
    }
}
