use serde::{Deserialize, Serialize};
use tracing::info;
use crate::engine::context::ExecutionContext;
use crate::engine::layers::base::{LayerOutput, LegalLayer};
use crate::engine::EngineError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynthesisOutput {
    pub consensus: Vec<String>,
    pub conflicts: Vec<String>,
    pub layer_weights: Vec<(u8, f32)>,
    pub final_interpretation: String,
    pub certainty: f32,
    pub primary_risks: Vec<String>,
    pub recommendations: Vec<String>,
}

pub struct SynthesisEngine;

impl SynthesisEngine {
    pub fn new() -> Self {
        Self
    }

    fn compute_weights(&self, mode: &crate::engine::mode_router::ReasoningMode) -> Vec<(u8, f32)> {
        match mode {
            crate::engine::mode_router::ReasoningMode::Exploration => vec![
                (1, 0.15), (2, 0.15), (3, 0.10), (4, 0.10), (5, 0.10), (6, 0.20), (7, 0.20),
            ],
            crate::engine::mode_router::ReasoningMode::Preventive => vec![
                (1, 0.15), (2, 0.20), (3, 0.15), (4, 0.10), (5, 0.10), (6, 0.15), (7, 0.15),
            ],
            crate::engine::mode_router::ReasoningMode::Adversarial => vec![
                (1, 0.15), (2, 0.15), (3, 0.10), (4, 0.10), (5, 0.10), (6, 0.20), (7, 0.20),
            ],
            crate::engine::mode_router::ReasoningMode::Neutral => vec![
                (1, 0.15), (2, 0.15), (3, 0.15), (4, 0.10), (5, 0.10), (6, 0.20), (7, 0.15),
            ],
            _ => vec![
                (1, 0.15), (2, 0.20), (3, 0.15), (4, 0.10), (5, 0.10), (6, 0.15), (7, 0.15),
            ],
        }
    }

    pub fn synthesize(
        &self,
        layers: &[LayerOutput],
        context: &ExecutionContext,
    ) -> Result<SynthesisOutput, EngineError> {
        info!(case_id=%context.case_id, layers_count=layers.len(), "synthesizing 7-layer interpretation");

        let mut consensus = Vec::new();
        let mut conflicts = Vec::new();
        let mut all_key_findings = Vec::new();

        for layer in layers {
            if !layer.key_findings.is_empty() {
                consensus.push(format!("[Layer {}: {}] {}", layer.layer_number, layer.layer_name, layer.key_findings[0]));
            }
            all_key_findings.extend(layer.key_findings.clone());
            if !layer.ambiguities.is_empty() {
                conflicts.push(format!("[Layer {}] {}", layer.layer_number, layer.ambiguities[0]));
            }
        }

        let weights = self.compute_weights(&context.reasoning_mode);

        let weighted_certainty: f32 = layers.iter().zip(weights.iter()).map(|(l, (_, w))| l.certainty * w).sum();
        let total_weight: f32 = weights.iter().map(|(_, w)| w).sum();
        let final_certainty = if total_weight > 0.0 { weighted_certainty / total_weight } else { 0.5 };

        let final_interpretation = format!(
            "SINTESIS 7 LAPISAN PENAFSIRAN\n\
             =========================\n\n\
             Berdasarkan analisis 7 lapisan penafsiran untuk mode {:?}:\n\n\
             Konsensus Antar Lapisan:\n{}\n\n\
             Konflik Antar Lapisan:\n{}\n\n\
             Tingkat Kepastian Akhir: {:.2}\n\n\
             Interpretasi Final:\n\
             Analisis multi-lapisan menunjukkan bahwa posisi hukum memiliki dasar yang cukup kuat, \
             namun masih terdapat area ketidakpastian yang memerlukan klarifikasi lebih lanjut dan \
             pengumpulan bukti pendukung. Rekomendasi: lanjutkan dengan mitigasi risiko yang telah diidentifikasi.",
            context.reasoning_mode,
            consensus.join("\n"),
            if conflicts.is_empty() { "Tidak ada konflik signifikan antar lapisan".to_string() } else { conflicts.join("\n") },
            final_certainty
        );

        let recommendations = vec![
            "Kumpulkan dan verifikasi bukti pendukung untuk memperkuat posisi".to_string(),
            "Lakukan mitigasi risiko yang telah diidentifikasi sebelum melanjutkan".to_string(),
            "Konsultasikan dengan ahli hukum untuk validasi lebih lanjut".to_string(),
            "Pertimbangkan alternatif penyelesaian selain litigasi".to_string(),
        ];

        let primary_risks = context.case_graph.risks.iter().take(3).map(|r| r.content.clone()).collect();

        Ok(SynthesisOutput {
            consensus,
            conflicts,
            layer_weights: weights,
            final_interpretation,
            certainty: final_certainty,
            primary_risks,
            recommendations,
        })
    }
}
