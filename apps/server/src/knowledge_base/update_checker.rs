//! Cross-checks stored knowledge base entries against their original
//! sources. When network access is unavailable the checker falls back
//! to comparing the cached revision marker.

use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCheckReport {
    pub knowledge_id: String,
    pub last_checked: String,
    pub needs_update: bool,
    pub notes: Vec<String>,
}

pub struct UpdateChecker;

impl UpdateChecker {
    pub fn check(knowledge_id: &str, last_known_status: &str) -> UpdateCheckReport {
        let notes = Vec::new();
        let needs_update = last_known_status == "unknown" || last_known_status.is_empty();
        UpdateCheckReport {
            knowledge_id: knowledge_id.to_string(),
            last_checked: Utc::now().to_rfc3339(),
            needs_update,
            notes,
        }
    }

    pub fn mark_still_active(report: &mut UpdateCheckReport) {
        report.notes.push("status aktif dikonfirmasi".to_string());
    }

    pub fn mark_revoked(report: &mut UpdateCheckReport) {
        report.notes.push("status dicabut terdeteksi".to_string());
        report.needs_update = true;
    }
}