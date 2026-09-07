//! Knowledge base manager — orchestrates indexing, retrieval, and
//! lifecycle of legal documents within the local store.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::article_splitter::ArticleSplitter;
use super::embedding_generator::EmbeddingGenerator;
use super::hybrid_search::HybridSearch;
use super::keyword_search::KeywordSearch;
use super::metadata_extractor::MetadataExtractor;
use super::semantic_search::{IndexedDocument, SemanticHit, SemanticSearch};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeEntry {
    pub id: String,
    pub title: String,
    pub full_text: String,
    pub citation: String,
    pub metadata: serde_json::Value,
    pub created_at: String,
}

pub struct KnowledgeBaseManager {
    pub hybrid: HybridSearch,
    pub semantic: SemanticSearch,
    pub embeddings: EmbeddingGenerator,
    pub splitter: ArticleSplitter,
    pub metadata: MetadataExtractor,
    index: Vec<IndexedDocument>,
    entries: Vec<KnowledgeEntry>,
}

impl Default for KnowledgeBaseManager {
    fn default() -> Self {
        Self {
            hybrid: HybridSearch,
            semantic: SemanticSearch::default(),
            embeddings: EmbeddingGenerator::default(),
            splitter: ArticleSplitter::default(),
            metadata: MetadataExtractor::default(),
            index: Vec::new(),
            entries: Vec::new(),
        }
    }
}

impl KnowledgeBaseManager {
    pub fn ingest(&mut self, title: &str, full_text: &str, citation: &str, metadata: serde_json::Value) -> KnowledgeEntry {
        let id = Uuid::new_v4().to_string();
        let entry = KnowledgeEntry {
            id: id.clone(),
            title: title.to_string(),
            full_text: full_text.to_string(),
            citation: citation.to_string(),
            metadata,
            created_at: Utc::now().to_rfc3339(),
        };
        let indexed = self.semantic.index(&entry.id, &entry.title, &entry.full_text);
        self.index.push(indexed);
        self.entries.push(entry.clone());
        entry
    }

    pub fn search(&self, query: &str) -> Vec<SemanticHit> {
        self.semantic.query(query, &self.index)
    }

    pub fn hybrid_search(&self, query: &str) -> Vec<super::hybrid_search::HybridResult> {
        let docs: Vec<(String, String, String)> = self
            .entries
            .iter()
            .map(|entry| (entry.id.clone(), entry.title.clone(), entry.full_text.clone()))
            .collect();
        HybridSearch::search(query, &docs)
    }

    pub fn keyword_score(&self, query: &str, entry_id: &str) -> f32 {
        if let Some(entry) = self.entries.iter().find(|e| e.id == entry_id) {
            KeywordSearch::score(query, &entry.full_text)
        } else {
            0.0
        }
    }

    pub fn split_articles(&self, text: &str) -> Vec<super::article_splitter::ArticleChunk> {
        self.splitter.split(text)
    }

    pub fn extract_metadata(&self, text: &str) -> super::metadata_extractor::RegulationMetadata {
        self.metadata.extract(text)
    }

    pub fn list(&self) -> &[KnowledgeEntry] {
        &self.entries
    }

    pub fn get(&self, id: &str) -> Option<&KnowledgeEntry> {
        self.entries.iter().find(|entry| entry.id == id)
    }
}