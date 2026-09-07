use serde::{Deserialize, Serialize};
use tracing::info;
use crate::engine::context::{ExecutionContext, MessageTurn, MessageRole};
use crate::engine::nodes::{NodeExecutor, NodeOutput, NodeResult};
use crate::engine::EngineError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedFact {
    pub id: String,
    pub content: String,
    pub category: FactCategory,
    pub source_turn: usize,
    pub verified: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FactCategory {
    Event,
    Party,
    Timeline,
    Relationship,
    Transaction,
    Damage,
    Other,
}

pub struct FactExtractor;

impl FactExtractor {
    pub fn new() -> Self {
        Self
    }

    pub fn extract_from_message(message: &str, turn_index: usize) -> Vec<ExtractedFact> {
        let mut facts = Vec::new();
        let lower = message.to_lowercase();

        for (idx, sentence) in message.split(['.', '!', '?', '\n']).enumerate() {
            let trimmed = sentence.trim();
            if trimmed.is_empty() || trimmed.len() < 5 {
                continue;
            }

            let category = Self::categorize_fact(trimmed, &lower);
            let verified = Self::assess_verification(trimmed);

            facts.push(ExtractedFact {
                id: format!("fact_{}_{}", turn_index, idx),
                content: trimmed.to_string(),
                category,
                source_turn: turn_index,
                verified,
            });
        }

        facts
    }

    fn categorize_fact(sentence: &str, lower: &str) -> FactCategory {
        if lower.contains("pihak") || lower.contains("orang") || lower.contains("perusahaan") {
            return FactCategory::Party;
        }
        if lower.contains("tanggal") || lower.contains("tahun") || lower.contains("waktu") || lower.contains("hari") {
            return FactCategory::Timeline;
        }
        if lower.contains("hubungan") || lower.contains("izin") || lower.contains("status") {
            return FactCategory::Relationship;
        }
        if lower.contains("transfer") || lower.contains("bayar") || lower.contains("uang") || lower.contains("kontrak") {
            return FactCategory::Transaction;
        }
        if lower.contains("kerugian") || lower.contains("rugi") || lower.contains("kerusakan") || lower.contains("cedera") {
            return FactCategory::Damage;
        }
        if lower.contains("kejadian") || lower.contains("peristiwa") || lower.contains("terjadi") || lower.contains("telah") {
            return FactCategory::Event;
        }
        FactCategory::Other
    }

    fn assess_verification(sentence: &str) -> bool {
        let indicators = ["sesuai", "bukti", "dokumen", "konfirmasi", "saksi", "tercatat"];
        indicators.iter().any(|ind| sentence.to_lowercase().contains(ind))
    }
}

#[async_trait::async_trait]
impl NodeExecutor for FactExtractor {
    async fn execute(&self, context: &mut ExecutionContext) -> Result<NodeResult, EngineError> {
        context.check_cancelled()?;
        info!(case_id=%context.case_id, "extracting facts from conversation");

        let mut all_facts = Vec::new();
        for (idx, turn) in context.conversation_history.iter().enumerate() {
            if matches!(turn.role, MessageRole::User) {
                let facts = Self::extract_from_message(&turn.content, idx);
                all_facts.extend(facts);
            }
        }

        let current_facts: Vec<String> = all_facts.iter().map(|f| f.content.clone()).collect();
        for fact in &current_facts {
            context.case_graph.facts.push(crate::engine::context::GraphNode {
                id: format!("fact_{}", uuid::Uuid::new_v4()),
                node_type: "fact".to_string(),
                content: fact.clone(),
                metadata: serde_json::json!({"category": "user_stated", "verified": false}),
                certainty: 0.6,
            });
        }

        Ok(NodeResult {
            node_id: format!("fact_extraction_{}", uuid::Uuid::new_v4()),
            node_type: "fact_extractor".to_string(),
            success: true,
            output: NodeOutput::Facts {
                facts: all_facts.into_iter().map(|f| crate::engine::nodes::fact_extractor::FactEntry {
                    id: f.id,
                    content: f.content,
                    category: format!("{:?}", f.category).to_lowercase(),
                    source_turn: f.source_turn,
                    verified: f.verified,
                }).collect(),
            },
            confidence: 0.6,
            warnings: Vec::new(),
            metadata: serde_json::json!({"total_facts": current_facts.len()}),
        })
    }

    fn node_type(&self) -> &'static str {
        "fact_extractor"
    }
}
