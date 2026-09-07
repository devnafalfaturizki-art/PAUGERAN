use tracing::info;
use crate::engine::context::ExecutionContext;
use crate::engine::nodes::{NodeExecutor, NodeOutput, NodeResult, ArgumentEntry, ArgumentStrength, CounterargumentEntry};
use crate::engine::layers::base::{LegalLayer, LayerOutput};
use crate::engine::EngineError;

pub struct AdversarialMode;

impl AdversarialMode {
    pub fn new() -> Self {
        Self
    }

    fn identify_attack_vectors(&self, context: &ExecutionContext) -> Vec<(String, String, String)> {
        let mut attacks = Vec::new();

        for arg in &context.case_graph.arguments {
            attacks.push((
                format!("Serangan terhadap argumen: {}", arg.content),
                "Lawan dapat menyangkal kekuatan bukti pendukung".to_string(),
                "Siapkan bukti tambahan dan saksi".to_string(),
            ));
        }

        for rule in &context.case_graph.rules {
            if rule.metadata.get("status").and_then(|v| v.as_str()) == Some("amended") {
                attacks.push((
                    format!("Serangan terhadap peraturan yang diamendemen: {}", rule.content),
                    "Lawan dapat berargumen bahwa peraturan yang diamendemen tidak berlaku".to_string(),
                    "Verifikasi tanggal peristiwa dan aturan peralihan".to_string(),
                ));
            }
        }

        for conclusion in &context.case_graph.conclusions {
            if conclusion.certainty < 0.5 {
                attacks.push((
                    format!("Serangan terhadap kesimpulan lemah: {}", conclusion.content),
                    "Lawan dapat menyerang asumsi yang menjadi dasar kesimpulan".to_string(),
                    "Perkuat asumsi dengan bukti lebih kuat".to_string(),
                ));
            }
        }

        if attacks.is_empty() {
            attacks.push((
                "Serangan umum terhadap fondasi kasus".to_string(),
                "Lawan dapat mencari celah dalam rantai pembuktian".to_string(),
                "Pastikan setiap kesimpulan memiliki rantai bukti yang kuat".to_string(),
            ));
        }

        attacks.truncate(10);
        attacks
    }

    async fn run_adversarial_layers(&self, context: &mut ExecutionContext) -> Result<Vec<LayerOutput>, EngineError> {
        let mut layers = Vec::new();

        let grammatical = crate::engine::engine::layers::grammatical::GrammaticalInterpreter::new();
        layers.push(grammatical.analyze(context).await?);

        let systematic = crate::engine::engine::layers::systematic::SystematicInterpreter::new();
        layers.push(systematic.analyze(context).await?);

        let critical = crate::engine::engine::layers::critical::CriticalInterpreter::new();
        layers.push(critical.analyze(context).await?);

        let comparative = crate::engine::engine::layers::comparative::ComparativeInterpreter::new();
        layers.push(comparative.analyze(context).await?);

        layers
    }
}

#[async_trait::async_trait]
impl NodeExecutor for AdversarialMode {
    async fn execute(&self, context: &mut ExecutionContext) -> Result<NodeResult, EngineError> {
        context.check_cancelled()?;
        info!(case_id=%context.case_id, mode="adversarial", "executing adversarial mode");

        let layers = Self::run_adversarial_layers(self, context).await?;
        let attack_vectors = Self::identify_attack_vectors(self, context);

        let synthesis = crate::engine::engine::layers::synthesis::SynthesisEngine::new()
            .synthesize(&layers, context)?;

        let interpretation = format!(
            "MODE ADVERSARIAL — Devil's Advocate Analysis & Stress Test\n\
             =======================================================\n\n\
             VEKTOR SERANGAN YANG MUNGKIN DIGUNAKAN LAWAN:\n{}\n\n\
             Tingkat kepastian: {:.0}%\n\n\
             Rekomendasi: Perkuat argumen yang lemah dan antisipasi serangan lawan.",
            attack_vectors.iter().enumerate().map(|(i, (a, t, m))| format!("{}. {}\n   Taktik: {}\n   Mitigasi: {}", i + 1, a, t, m)).collect::<Vec<_>>().join("\n\n"),
            synthesis.certainty * 100.0
        );

        Ok(NodeResult {
            node_id: format!("adversarial_{}", uuid::Uuid::new_v4()),
            node_type: "mode_adversarial".to_string(),
            success: true,
            output: NodeOutput::Clarification { questions: vec!["Apa kelemahan paling mungkin diserang lawan?".to_string()] },
            confidence: synthesis.certainty,
            warnings: attack_vectors.iter().map(|(a, _, _)| a.clone()).collect(),
            metadata: serde_json::json!({"interpretation": interpretation, "attack_vectors": attack_vectors, "certainty": synthesis.certainty}),
        })
    }

    fn node_type(&self) -> &'static str {
        "mode_adversarial"
    }
}
