use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct PreferenceRecord {
    pub id: String,
    pub user_id: String,
    pub theme: String,
    pub font_size: String,
    pub language: String,
    pub updated_at: String,
}

impl Default for PreferenceRecord {
    fn default() -> Self {
        Self {
            id: "global".into(),
            user_id: "global".into(),
            theme: "light".into(),
            font_size: "medium".into(),
            language: "id".into(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}
