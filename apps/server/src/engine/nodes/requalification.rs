use tracing::info;
use crate::engine::context::ExecutionContext;
use crate::engine::nodes::{NodeExecutor, NodeOutput, NodeResult, RequalificationChallenge};
use crate::engine::EngineError;

pub struct FactRequalifier;

impl FactRequalifier {
    pub fn new() -> Self {
        Self
    }

    fn challenge_facts(&self, context: &ExecutionContext) -> Vec<RequalificationChallenge> {
        let mut challenges = Vec::new();

        for fact in &context.case_graph.facts {
            let content_lower = fact.content.to_lowercase();

            if content_lower.contains("perjanjian") && !content_lower.contains("tertulis") {
                challenges.push(RequalificationChallenge {
                    original_qualification: "Perjanjian".to_string(),
                    suggested_qualification: "Surat perjanjian di bawah tangan atau perikatan yang timbul dari perbuatan hukum".to_string(),
                    legal_basis: "Pasal 1320 KUHPerdata".to_string(),
                    explanation: "Perjanjian lisan atau di bawah tangan memiliki kekuatan pembuktian yang berbeda dengan akta notariil. Pastikan kualifikasi yang tepat untuk menentukan prosedur pembuktian.".to_string(),
                });
            }

            if content_lower.contains("pekerja") || content_lower.contains("karyawan") {
                challenges.push(RequalificationChallenge {
                    original_qualification: "Pekerja".to_string(),
                    suggested_qualification: "Pegawai sesuai UU Ketenagakerjaan atau pekerja lepas sesuai ketentuan yang berlaku".to_string(),
                    legal_basis: "UU No. 13 Tahun 2003 tentang Ketenagakerjaan".to_string(),
                    explanation: "Kualifikasi 'pekerja' vs 'pegawai' vs 'pekerja lepas' memiliki konsekuensi hukum yang sangat berbeda. Verifikasi status hubungan kerja yang sebenarnya.".to_string(),
                });
            }

            if content_lower.contains("kerugian") || content_lower.contains("rugi") {
                challenges.push(RequalificationChallenge {
                    original_qualification: "Kerugian".to_string(),
                    suggested_qualification: "Kerugian materiel vs immateriel, atau keuntungan yang seharusnya diperoleh (lucrum cessans)".to_string(),
                    legal_basis: "Pasal 1246 KUHPerdata".to_string(),
                    explanation: "Kategori kerugian menentukan jenis ganti rugi yang dapat dituntut dan cara pembuktiannya. Perbedaan antara damnum emergens dan lucrum cessans penting untuk perhitungan ganti rugi.".to_string(),
                });
            }

            if content_lower.contains("perusahaan") || content_lower.contains("pt") {
                challenges.push(RequalificationChallenge {
                    original_qualification: "Perusahaan".to_string(),
                    suggested_qualification: "PT, CV, usaha perseorangan, atau badan hukum lain sesuai akta pendirian".to_string(),
                    legal_basis: "UU No. 40 Tahun 2007 tentang Perseroan Terbatas".to_string(),
                    explanation: "Status badan hukum mempengaruhi kewenangan pengurus, tanggung jawab, dan prosedur hukum yang berlaku.".to_string(),
                });
            }
        }

        if challenges.is_empty() {
            challenges.push(RequalificationChallenge {
                original_qualification: "Fakta".to_string(),
                suggested_qualification: "Verifikasi ulang kualifikasi hukum fakta yang disampaikan".to_string(),
                legal_basis: "Prasangka hukum (iuris praesumptio)".to_string(),
                explanation: "Fakta yang disampaikan pengguna mungkin belum dalam terminologi hukum yang tepat. Rekomendasi: verifikasi ulang dengan ahli hukum.".to_string(),
            });
        }

        challenges
    }
}

#[async_trait::async_trait]
impl NodeExecutor for FactRequalifier {
    async fn execute(&self, context: &mut ExecutionContext) -> Result<NodeResult, EngineError> {
        context.check_cancelled()?;
        info!(case_id=%context.case_id, "performing fact re-qualification challenge");

        let challenges = Self::challenge_facts(self, context);

        Ok(NodeResult {
            node_id: format!("requal_{}", uuid::Uuid::new_v4()),
            node_type: "requalification".to_string(),
            success: true,
            output: NodeOutput::Requalification { challenges },
            confidence: 0.7,
            warnings: Vec::new(),
            metadata: serde_json::json!({"total_challenges": challenges.len()}),
        })
    }

    fn node_type(&self) -> &'static str {
        "requalification"
    }
}
