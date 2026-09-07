use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct AuditRecord {
    pub id: String,
    pub action: String,
    pub user_id: String,
    pub case_id: String,
    pub details: String,
    pub created_at: String,
}
