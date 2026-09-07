//! Maintains a curated map of known norm conflicts between Indonesian
//! regulations so the engine can apply lex superior/specialis/posterior
//! consistently.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormConflictEntry {
    pub older_regulation: String,
    pub newer_regulation: String,
    pub topic: String,
    pub resolution_rule: String,
    pub notes: Option<String>,
}

#[derive(Default)]
pub struct NormConflictDatabase {
    entries: Vec<NormConflictEntry>,
    by_pair: HashMap<(String, String), usize>,
}

impl NormConflictDatabase {
    pub fn register(&mut self, entry: NormConflictEntry) {
        let pair = (entry.older_regulation.clone(), entry.newer_regulation.clone());
        if let Some(index) = self.by_pair.get(&pair).copied() {
            self.entries[index] = entry;
        } else {
            self.by_pair.insert(pair, self.entries.len());
            self.entries.push(entry);
        }
    }

    pub fn lookup(&self, older: &str, newer: &str) -> Option<&NormConflictEntry> {
        self.by_pair
            .get(&(older.to_string(), newer.to_string()))
            .and_then(|index| self.entries.get(*index))
    }

    pub fn all(&self) -> &[NormConflictEntry] {
        &self.entries
    }

    pub fn known_conflicts_for(&self, regulation_id: &str) -> Vec<&NormConflictEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.older_regulation == regulation_id || entry.newer_regulation == regulation_id)
            .collect()
    }
}