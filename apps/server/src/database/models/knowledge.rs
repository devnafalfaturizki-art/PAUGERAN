use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// API-facing representation of a knowledge-base entry.
/// `tags` is exposed as a vector even though the database stores it
/// as a delimited string for portability across SQLite versions.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KnowledgeEntry {
    pub id: String,
    pub title: String,
    pub content: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub created_at: String,
}

/// Internal persistence record (comma-delimited tags).
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct KnowledgeRecord {
    pub id: String,
    pub title: String,
    pub content: String,
    pub tags: String,
    pub created_at: String,
}

/// Payload accepted by the knowledge-base endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewKnowledgeEntry {
    pub title: String,
    pub content: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

impl NewKnowledgeEntry {
    pub fn validate(&self) -> Result<(), String> {
        if self.title.trim().is_empty() {
            return Err("title wajib diisi".into());
        }
        if self.content.trim().is_empty() {
            return Err("content wajib diisi".into());
        }
        Ok(())
    }
}

impl KnowledgeEntry {
    pub fn from_record(record: KnowledgeRecord) -> Self {
        let tags = record
            .tags
            .split(',')
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
            .collect();
        Self {
            id: record.id,
            title: record.title,
            content: record.content,
            tags,
            created_at: record.created_at,
        }
    }
}

impl KnowledgeRecord {
    pub fn from_entry(entry: KnowledgeEntry) -> Self {
        Self {
            id: entry.id,
            title: entry.title,
            content: entry.content,
            tags: entry.tags.join(","),
            created_at: entry.created_at,
        }
    }
}