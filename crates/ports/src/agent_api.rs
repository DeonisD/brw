use async_trait::async_trait;
use domain::agent::Agent;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AgentError {
    #[error("Агент не найден: {0}")]
    NotFound(String),

    #[error("Ошибка базы данных: {0}")]
    Database(String),
}

/// Трейт — аналог интерфейса AgentResources
#[async_trait]
pub trait AgentApi: Send + Sync {
    async fn get_all_agents(&self) -> Result<Vec<Agent>, AgentError>;
}
