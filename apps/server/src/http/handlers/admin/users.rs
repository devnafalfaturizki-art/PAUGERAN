//! User management handlers.
//! 
//! [CB §27] — Multi-User Authentication

use axum::{
    extract::State,
    Json,
};
use std::sync::Arc;

use crate::database::repositories::UserRepository;
use crate::error::AppError;
use crate::http::state::AppState;

/// List all users in the team.
/// 
/// [CB §27] — Admin can view team members
pub async fn list_users(
    State(_state): State<Arc<AppState>>,
) -> Result<Json<Vec<crate::database::models::User>>, AppError> {
    let repo = UserRepository::new();
    let users = repo.find_all()
        .await
        .map_err(AppError::Database)?;
    Ok(Json(users))
}

/// Create a new user (admin only).
/// 
/// [CB §27] — First user is admin
pub async fn create_user(
    State(_state): State<Arc<AppState>>,
    Json(_payload): Json<crate::database::models::NewUser>,
) -> Result<Json<crate::database::models::User>, AppError> {
    let repo = UserRepository::new();
    let user = repo.create(_payload)
        .await
        .map_err(AppError::Database)?;
    Ok(Json(user))
}

/// Delete a user (admin only).
/// 
/// [CB §27] — Admin manages team
pub async fn delete_user(
    State(_state): State<Arc<AppState>>,
    axum::extract::Path(user_id): axum::extract::Path<String>,
) -> Result<(), AppError> {
    let repo = UserRepository::new();
    repo.delete(&user_id)
        .await
        .map_err(AppError::Database)?;
    Ok(())
}
