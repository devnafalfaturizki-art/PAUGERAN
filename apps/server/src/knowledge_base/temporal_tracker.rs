//! Tracks effective-date windows for Indonesian regulations so the
//! engine can apply the correct statute to a given event.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulationWindow {
    pub regulation_id: String,
    pub title: String,
    pub effective_from: DateTime<Utc>,
    pub effective_until: Option<DateTime<Utc>>,
    pub status: String,
}

pub struct TemporalLawTracker {
    windows: Vec<RegulationWindow>,
}

impl Default for TemporalLawTracker {
    fn default() -> Self {
        Self { windows: Vec::new() }
    }
}

impl TemporalLawTracker {
    pub fn register(&mut self, window: RegulationWindow) {
        self.windows.retain(|existing| existing.regulation_id != window.regulation_id);
        self.windows.push(window);
    }

    pub fn applicable(&self, regulation_id: &str, event_date: DateTime<Utc>) -> Option<&RegulationWindow> {
        self.windows
            .iter()
            .find(|window| {
                window.regulation_id == regulation_id
                    && window.effective_from <= event_date
                    && window.effective_until.map(|until| event_date <= until).unwrap_or(true)
            })
    }

    pub fn superseded(&self, regulation_id: &str) -> bool {
        self.windows
            .iter()
            .any(|window| window.regulation_id == regulation_id && window.status == "dicabut")
    }

    pub fn list(&self) -> &[RegulationWindow] {
        &self.windows
    }

    pub fn active_at(&self, event_date: DateTime<Utc>) -> Vec<&RegulationWindow> {
        self.windows
            .iter()
            .filter(|window| {
                window.effective_from <= event_date
                    && window.effective_until.map(|until| event_date <= until).unwrap_or(true)
                    && window.status != "dicabut"
            })
            .collect()
    }
}