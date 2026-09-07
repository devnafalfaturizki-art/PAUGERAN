use crate::persistence::Database;
use crate::database::models::document::DocumentRecord;

#[derive(Clone)]
pub struct DocumentRepository {
    database: Database,
}

impl DocumentRepository {
    pub fn new(database: Database) -> Self {
        Self { database }
    }

    pub async fn list(&self, case_id: &str) -> Result<Vec<DocumentRecord>, sqlx::Error> {
        self.database.list_documents(case_id).await
    }

    pub async fn save(&self, record: &DocumentRecord) -> Result<(), sqlx::Error> {
        self.database.save_document(record).await
    }
}
