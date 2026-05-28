use axum::{extract::State, Json};
use ports::agent_api::AgentApi;
use std::sync::Arc;

use crate::{error::AppError, state::AppState};

/// GET /agents
/// Аналог AgentResources.agents()
/// @AgentRequired добавляется в router.rs
#[axum::debug_handler]
pub async fn get_agents(
    State(state): State<AppState>,
    // Здесь позже будет внедрение Arc<dyn AgentApi>
) -> Result<Json<Vec<domain::agent::Agent>>, AppError> {
    tracing::info!("Запрос списка агентов");

    // Временно — прямой запрос к БД (позже заменим на сервис)
    let agents = sqlx::query_as!(
        domain::agent::Agent,
        r#"
        SELECT id, key, name, status
        FROM agents
        ORDER BY id
        "#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok(Json(agents))
}
