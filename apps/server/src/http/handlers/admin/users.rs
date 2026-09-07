//! User management handlers.
//!
//! [CB §27] — Multi-User Authentication

use axum::{
    extract::{Path, State},
    Json,
};
use std::sync::Arc;

use crate::crypto::password_hash::hash_password;
use crate::database::models::user::{NewUser, User, UserRecord};
use crate::error::AppError;
use crate::http::state::AppState;
use crate::utils::id_generator::new_uuid;

fn to_public(record: UserRecord) -> User {
    User {
        id: record.id,
        email: record.email,
        role: record.role,
        created_at: record.created_at,
    }
}

/// List all users in the team.
///
/// [CB §27] — Admin can view team members (metadata only).
pub async fn list_users(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<User>>, AppError> {
    let rows = sqlx::query_as::<_, UserRecord>(
        "SELECT id, email, password_hash, role, created_at FROM users ORDER BY created_at",
    )
    .fetch_all(state.database.pool())
    .await
    .map_err(AppError::Database)?;

    Ok(Json(rows.into_iter().map(to_public).collect()))
}

/// Create a new user (admin only).
///
/// [CB §27] — First user is admin.
pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<NewUser>,
) -> Result<Json<User>, AppError> {
    payload
        .validate()
        .map_err(|message| AppError::Validation(message))?;

    let password_hash = hash_password(&payload.password)
        .map_err(|error| AppError::Internal(format!("gagal hash password: {error}")))?;

    let record = UserRecord {
        id: new_uuid(),
        email: payload.email.trim().to_string(),
        password_hash,
        role: payload.role.unwrap_or_else(|| "member".into()),
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    sqlx::query(
        "INSERT INTO users (id, email, password_hash, role, created_at) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&record.id)
    .bind(&record.email)
    .bind(&record.password_hash)
    .bind(&record.role)
    .bind(&record.created_at)
    .execute(state.database.pool())
    .await
    .map_err(|error| match error {
        sqlx::Error::Database(db) if db.is_unique_violation() => {
            AppError::Validation("email sudah digunakan".into())
        }
        other => AppError::Database(other),
    })?;

    Ok(Json(to_public(record)))
}

/// Delete a user (admin only).
///
/// [CB §27] — Admin manages team.
pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<String>,
) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(&user_id)
        .execute(state.database.pool())
        .await
        .map_err(AppError::Database)?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("user tidak ditemukan".into()));
    }
    Ok(())
}