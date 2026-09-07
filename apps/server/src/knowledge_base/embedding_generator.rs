//! Lightweight embedding generator for Indonesian legal documents.
//!
//! Uses a deterministic hashing trick to produce dense vector
//! representations of documents without requiring an external model
//! service. This is intentionally simple: real semantic embeddings
//! can be added later behind the same interface.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct EmbeddingGenerator {
    pub dimensions: usize,
}

impl Default for EmbeddingGenerator {
    fn default() -> Self {
        Self { dimensions: 128 }
    }
}

impl EmbeddingGenerator {
    pub fn embed_text(&self, text: &str) -> Vec<f32> {
        let mut vector = vec![0.0f32; self.dimensions];
        for token in text.split(|ch: char| !ch.is_alphanumeric()) {
            if token.is_empty() {
                continue;
            }
            let mut hasher = DefaultHasher::new();
            token.to_lowercase().hash(&mut hasher);
            let bucket = (hasher.finish() as usize) % self.dimensions;
            vector[bucket] += 1.0;
        }
        self.normalize(&mut vector);
        vector
    }

    pub fn embed_documents(&self, documents: &[String]) -> Vec<Vec<f32>> {
        documents.iter().map(|doc| self.embed_text(doc)).collect()
    }

    pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
        if a.len() != b.len() || a.is_empty() {
            return 0.0;
        }
        let mut dot = 0.0f32;
        let mut norm_a = 0.0f32;
        let mut norm_b = 0.0f32;
        for (x, y) in a.iter().zip(b.iter()) {
            dot += x * y;
            norm_a += x * x;
            norm_b += y * y;
        }
        let denom = (norm_a.sqrt() * norm_b.sqrt()).max(f32::EPSILON);
        dot / denom
    }

    fn normalize(&self, vector: &mut [f32]) {
        let norm: f32 = vector.iter().map(|v| v * v).sum::<f32>().sqrt();
        if norm <= f32::EPSILON {
            return;
        }
        for value in vector.iter_mut() {
            *value /= norm;
        }
    }
}