use async_trait::async_trait;
use db::agent_repo::AgentRepo;
use domain::agent::Agent;
use ports::agent_api::{AgentApi, AgentError};
use tracing::info;

pub struct AgentService {
    repo: AgentRepo,
}

impl AgentService {
    pub fn new(repo: AgentRepo) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl AgentApi for AgentService {
    async fn get_all_agents(&self) -> Result<Vec<Agent>, AgentError> {
        info!("Получение списка всех агентов");

        self.repo
            .find_all()
            .await
            .map_err(|e| AgentError::Database(e.to_string()))
    }
}
