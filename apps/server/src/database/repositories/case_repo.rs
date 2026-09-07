//! Case repository boundary.

use crate::persistence::{CaseRecord, Database};

#[derive(Clone)]
pub struct CaseRepository {
    database: Database,
}

impl CaseRepository {
    pub fn new(database: Database) -> Self {
        Self { database }
    }

    pub async fn list(&self) -> Result<Vec<CaseRecord>, sqlx::Error> {
        self.database.list_cases().await
    }
}
