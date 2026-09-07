use crate::database::Database;
use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct AppState {
    pub started_at: DateTime<Utc>,
    pub database: Database,
}
