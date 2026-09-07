use crate::crypto::pii_redactor::PiiRedactor;
use crate::database::Database;
use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct AppState {
    pub started_at: DateTime<Utc>,
    pub database: Database,
    pub pii_redactor: PiiRedactor,
}
