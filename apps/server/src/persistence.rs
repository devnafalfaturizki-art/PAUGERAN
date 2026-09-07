use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use uuid::Uuid;

use crate::case_graph::{CaseEdge, CaseNode};
use crate::engine::mode_router::ReasoningMode;
use crate::engine::state_machine::CaseState;

#[derive(Clone)]
pub struct Database {
    pool: SqlitePool,
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
        let url = format!("sqlite://{data_dir}/paugeran.db");
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(
                url.parse::<sqlx::sqlite::SqliteConnectOptions>()?
                    .create_if_missing(true),
            )
            .await?;

        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(Self { pool })
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

    pub fn parse_timestamp(value: &str) -> DateTime<Utc> {
        DateTime::parse_from_rfc3339(value)
            .map(|timestamp| timestamp.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now())
    }
}
