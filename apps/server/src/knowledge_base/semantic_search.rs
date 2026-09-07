//! Semantic search using vector embeddings and cosine similarity.
//!
//! Falls back to no results when no vectors have been registered.

use super::embedding_generator::EmbeddingGenerator;

#[derive(Debug, Clone)]
pub struct IndexedDocument {
    pub id: String,
    pub title: String,
    pub vector: Vec<f32>,
}

pub struct SemanticSearch {
    pub generator: EmbeddingGenerator,
}

impl Default for SemanticSearch {
    fn default() -> Self {
        Self { generator: EmbeddingGenerator::default() }
    }
}

#[derive(Debug, Clone)]
pub struct SemanticHit {
    pub id: String,
    pub title: String,
    pub score: f32,
}

impl SemanticSearch {
    pub fn index(&self, id: &str, title: &str, text: &str) -> IndexedDocument {
        IndexedDocument {
            id: id.to_string(),
            title: title.to_string(),
            vector: self.generator.embed_text(text),
        }
    }

    pub fn query(&self, query: &str, index: &[IndexedDocument]) -> Vec<SemanticHit> {
        let query_vector = self.generator.embed_text(query);
        let mut hits: Vec<SemanticHit> = index
            .iter()
            .map(|doc| SemanticHit {
                id: doc.id.clone(),
                title: doc.title.clone(),
                score: EmbeddingGenerator::cosine_similarity(&query_vector, &doc.vector),
            })
            .filter(|hit| hit.score > 0.0)
            .collect();
        hits.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        hits
    }
}