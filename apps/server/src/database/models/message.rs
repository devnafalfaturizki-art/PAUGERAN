use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct MessageRecord {
    pub id: String,
    pub case_id: String,
    pub role: String,
    pub content: String,
    pub certainty_score: Option<f32>,
    pub created_at: String,
}
