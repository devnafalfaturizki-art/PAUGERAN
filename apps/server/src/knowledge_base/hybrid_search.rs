//! Hybrid search combining keyword matching with simple relevance ranking.
//!
//! Provides a fallback semantic-like search using token overlap and
//! weighted scoring when embeddings are unavailable.

use serde::{Deserialize, Serialize};

use super::keyword_search::KeywordSearch;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridResult {
    pub document_id: String,
    pub title: String,
    pub score: f32,
    pub matched_terms: Vec<String>,
}

pub struct HybridSearch;

impl HybridSearch {
    pub fn search(query: &str, documents: &[(String, String, String)]) -> Vec<HybridResult> {
        let terms: Vec<String> = KeywordSearch::tokenize(query);
        if terms.is_empty() {
            return Vec::new();
        }

        let mut results = Vec::new();
        for (id, title, content) in documents {
            let lower_title = title.to_lowercase();
            let lower_content = content.to_lowercase();
            let mut score = 0.0f32;
            let mut matched = Vec::new();

            for term in &terms {
                let occurrences = lower_content.matches(term).count() as f32;
                if occurrences > 0.0 {
                    score += occurrences * 1.0;
                    matched.push(term.clone());
                }
                if lower_title.contains(term) {
                    score += 5.0;
                    if !matched.contains(term) {
                        matched.push(term.clone());
                    }
                }
            }

            if score > 0.0 {
                let normalized = score / (terms.len() as f32 * 6.0).max(1.0);
                results.push(HybridResult {
                    document_id: id.clone(),
                    title: title.clone(),
                    score: normalized.min(1.0),
                    matched_terms: matched,
                });
            }
        }

        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        results
    }
}