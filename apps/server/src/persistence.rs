use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::path::PathBuf;
use uuid::Uuid;

use crate::case_graph::{CaseEdge, CaseNode};
use crate::crypto::key_manager::{decrypt_api_key, encrypt_api_key};
use crate::engine::mode_router::ReasoningMode;
use crate::engine::state_machine::CaseState;

#[derive(Clone)]
pub struct Database {
    pool: SqlitePool,
    data_dir: PathBuf,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CaseRecord {
    pub id: String,
    pub title: String,
    pub state: String,
    pub mode: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct CaseNodeRecord {
    pub id: String,
    pub case_id: String,
    pub node_type: String,
    pub content: String,
    pub metadata: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct CaseEdgeRecord {
    pub id: String,
    pub case_id: String,
    pub source_node_id: String,
    pub target_node_id: String,
    pub edge_type: String,
    pub metadata: String,
    pub created_at: String,
}

impl Database {
    pub async fn connect(data_dir: &str) -> Result<Self, sqlx::Error> {
        std::fs::create_dir_all(data_dir).map_err(sqlx::Error::Io)?;
        Self::connect_inner(&format!("sqlite://{data_dir}/paugeran.db"), PathBuf::from(data_dir)).await
    }

    pub async fn connect_url(url: &str) -> Result<Self, sqlx::Error> {
        Self::connect_inner(url, PathBuf::from(".")).await
    }

    async fn connect_inner(url: &str, data_dir: PathBuf) -> Result<Self, sqlx::Error> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(
                url.parse::<sqlx::sqlite::SqliteConnectOptions>()?
                    .create_if_missing(true),
            )
            .await?;

        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(Self { pool, data_dir })
    }

    pub async fn list_cases(&self) -> Result<Vec<CaseRecord>, sqlx::Error> {
        sqlx::query_as::<_, CaseRecord>(
            "SELECT id, title, state, mode, updated_at FROM cases ORDER BY updated_at DESC",
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn find_case(&self, id: &str) -> Result<Option<CaseRecord>, sqlx::Error> {
        sqlx::query_as::<_, CaseRecord>(
            "SELECT id, title, state, mode, updated_at FROM cases WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn seed_if_empty(&self) -> Result<(), sqlx::Error> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM cases")
            .fetch_one(&self.pool)
            .await?;
        if count.0 == 0 {
            self.create_case("Sengketa perjanjian kerja", CaseState::Dispute, "dispute")
                .await?;
        }
        Ok(())
    }

    pub async fn create_case(
        &self,
        title: &str,
        state: CaseState,
        mode: &str,
    ) -> Result<CaseRecord, sqlx::Error> {
        let record = CaseRecord {
            id: Uuid::new_v4().to_string(),
            title: title.trim().to_string(),
            state: state.as_str().to_string(),
            mode: mode.to_string(),
            updated_at: Utc::now().to_rfc3339(),
        };
        sqlx::query(
            "INSERT INTO cases (id, title, state, mode, updated_at) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(&record.id)
        .bind(&record.title)
        .bind(&record.state)
        .bind(&record.mode)
        .bind(&record.updated_at)
        .execute(&self.pool)
        .await?;
        Ok(record)
    }

    pub async fn update_state(
        &self,
        id: &str,
        state: CaseState,
    ) -> Result<Option<CaseRecord>, sqlx::Error> {
        let state_name = state.as_str().to_string();
        let result = sqlx::query("UPDATE cases SET state = ?, updated_at = ? WHERE id = ?")
            .bind(state_name)
            .bind(Utc::now().to_rfc3339())
            .bind(id)
            .execute(&self.pool)
            .await?;
        if result.rows_affected() == 0 {
            return Ok(None);
        }
        sqlx::query_as::<_, CaseRecord>(
            "SELECT id, title, state, mode, updated_at FROM cases WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn update_mode(
        &self,
        id: &str,
        mode: ReasoningMode,
    ) -> Result<Option<CaseRecord>, sqlx::Error> {
        let result = sqlx::query("UPDATE cases SET mode = ?, updated_at = ? WHERE id = ?")
            .bind(mode.as_str())
            .bind(Utc::now().to_rfc3339())
            .bind(id)
            .execute(&self.pool)
            .await?;
        if result.rows_affected() == 0 {
            return Ok(None);
        }
        self.find_case(id).await
    }

    pub async fn save_message(
        &self,
        case_id: &str,
        role: &str,
        content: &str,
        certainty_score: Option<f32>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO messages (id, case_id, role, content, certainty_score, created_at)
             VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(Uuid::new_v4().to_string())
        .bind(case_id)
        .bind(role)
        .bind(content)
        .bind(certainty_score)
        .bind(Utc::now().to_rfc3339())
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn list_nodes(&self, case_id: &str) -> Result<Vec<CaseNodeRecord>, sqlx::Error> {
        sqlx::query_as::<_, CaseNodeRecord>("SELECT id, case_id, node_type, content, metadata, created_at FROM case_nodes WHERE case_id = ? ORDER BY created_at")
            .bind(case_id).fetch_all(&self.pool).await
    }

    pub async fn create_node(&self, node: &CaseNode) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO case_nodes (id, case_id, node_type, content, metadata, created_at) VALUES (?, ?, ?, ?, ?, ?)")
            .bind(&node.id).bind(&node.case_id).bind(node.node_type.as_str()).bind(&node.content)
            .bind(serde_json::to_string(&node.metadata).unwrap_or_else(|_| "{}".to_string()))
            .bind(Utc::now().to_rfc3339()).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn list_edges(&self, case_id: &str) -> Result<Vec<CaseEdgeRecord>, sqlx::Error> {
        sqlx::query_as::<_, CaseEdgeRecord>("SELECT id, case_id, source_node_id, target_node_id, edge_type, metadata, created_at FROM case_edges WHERE case_id = ? ORDER BY created_at")
            .bind(case_id).fetch_all(&self.pool).await
    }

    pub async fn create_edge(&self, edge: &CaseEdge) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO case_edges (id, case_id, source_node_id, target_node_id, edge_type, metadata, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)")
            .bind(&edge.id).bind(&edge.case_id).bind(&edge.source_node_id).bind(&edge.target_node_id)
            .bind(edge.edge_type.as_str()).bind(serde_json::to_string(&edge.metadata).unwrap_or_else(|_| "{}".to_string()))
            .bind(Utc::now().to_rfc3339()).execute(&self.pool).await?;
        Ok(())
    }

    pub async fn list_documents(&self, case_id: &str) -> Result<Vec<crate::database::models::document::DocumentRecord>, sqlx::Error> {
        sqlx::query_as::<_, crate::database::models::document::DocumentRecord>("SELECT id, case_id, filename, content_type, size, created_at FROM documents WHERE case_id = ? ORDER BY created_at")
            .bind(case_id).fetch_all(&self.pool).await
    }

    pub async fn save_document(&self, record: &crate::database::models::document::DocumentRecord) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT OR REPLACE INTO documents (id, case_id, filename, content_type, size, created_at) VALUES (?, ?, ?, ?, ?, ?)")
            .bind(&record.id).bind(&record.case_id).bind(&record.filename).bind(&record.content_type).bind(&record.size).bind(&record.created_at)
            .execute(&self.pool).await?;
        Ok(())
    }

    pub async fn list_knowledge(&self) -> Result<Vec<crate::database::models::knowledge::KnowledgeRecord>, sqlx::Error> {
        sqlx::query_as::<_, crate::database::models::knowledge::KnowledgeRecord>("SELECT id, title, content, tags, created_at FROM knowledge_base ORDER BY created_at DESC")
            .fetch_all(&self.pool).await
    }

    pub async fn save_knowledge(&self, record: &crate::database::models::knowledge::KnowledgeRecord) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT OR REPLACE INTO knowledge_base (id, title, content, tags, created_at) VALUES (?, ?, ?, ?, ?)")
            .bind(&record.id).bind(&record.title).bind(&record.content).bind(&record.tags).bind(&record.created_at)
            .execute(&self.pool).await?;
        Ok(())
    }

    pub async fn list_providers(&self) -> Result<Vec<crate::database::models::provider::ProviderRecord>, sqlx::Error> {
        let rows = sqlx::query_as::<_, (String, String, String, String, String, String)>(
            "SELECT id, name, provider_type, api_key, model, created_at FROM providers ORDER BY created_at DESC",
        )
        .fetch_all(&self.pool)
        .await?;

        let mut providers = Vec::with_capacity(rows.len());
        for (id, name, provider_type, encrypted_api_key, model, created_at) in rows {
            let api_key = match decrypt_api_key(&encrypted_api_key, &self.data_dir) {
                Ok(key) => key,
                Err(_) => String::new(),
            };
            providers.push(crate::database::models::provider::ProviderRecord {
                id,
                name,
                provider_type,
                api_key,
                model,
                created_at,
            });
        }
        Ok(providers)
    }

    pub async fn save_provider(&self, record: &crate::database::models::provider::ProviderRecord) -> Result<(), sqlx::Error> {
        let encrypted_api_key = encrypt_api_key(&record.api_key, &self.data_dir)
            .map_err(|e| sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;
        sqlx::query("INSERT OR REPLACE INTO providers (id, name, provider_type, api_key, model, created_at) VALUES (?, ?, ?, ?, ?, ?)")
            .bind(&record.id).bind(&record.name).bind(&record.provider_type).bind(&encrypted_api_key).bind(&record.model).bind(&record.created_at)
            .execute(&self.pool).await?;
        Ok(())
    }

    pub async fn get_preference(&self, user_id: &str) -> Result<Option<crate::database::models::preference::PreferenceRecord>, sqlx::Error> {
        sqlx::query_as::<_, crate::database::models::preference::PreferenceRecord>("SELECT id, user_id, theme, font_size, language, updated_at FROM preferences WHERE user_id = ?")
            .bind(user_id).fetch_optional(&self.pool).await
    }

    pub async fn save_preference(&self, record: &crate::database::models::preference::PreferenceRecord) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT OR REPLACE INTO preferences (id, user_id, theme, font_size, language, updated_at) VALUES (?, ?, ?, ?, ?, ?)")
            .bind(&record.id).bind(&record.user_id).bind(&record.theme).bind(&record.font_size).bind(&record.language).bind(&record.updated_at)
            .execute(&self.pool).await?;
        Ok(())
    }

    pub fn parse_timestamp(value: &str) -> DateTime<Utc> {
        DateTime::parse_from_rfc3339(value)
            .map(|timestamp| timestamp.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now())
    }
}
