//! Keyword-based full-text search for the knowledge base.
//!
//! Produces case-insensitive token matches and simple tf-style scoring
//! without requiring an external search engine.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeywordMatch {
    pub term: String,
    pub occurrences: usize,
}

pub struct KeywordSearch;

impl KeywordSearch {
    pub fn tokenize(input: &str) -> Vec<String> {
        input
            .split(|ch: char| !ch.is_alphanumeric())
            .filter(|segment| !segment.is_empty())
            .map(|segment| segment.to_lowercase())
            .collect()
    }

    pub fn match_terms(query: &str, content: &str) -> Vec<KeywordMatch> {
        let terms = Self::tokenize(query);
        let lower = content.to_lowercase();
        let mut matches = Vec::new();
        for term in terms {
            if term.len() < 2 {
                continue;
            }
            let count = lower.matches(&term).count();
            if count > 0 {
                matches.push(KeywordMatch {
                    term,
                    occurrences: count,
                });
            }
        }
        matches
    }

    pub fn score(query: &str, content: &str) -> f32 {
        let matches = Self::match_terms(query, content);
        if matches.is_empty() {
            return 0.0;
        }
        let total: usize = matches.iter().map(|m| m.occurrences).sum();
        (total as f32).sqrt()
    }
}