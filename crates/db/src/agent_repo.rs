use domain::agent::Agent;
use sqlx::PgPool;

/// Репозиторий агентов — аналог AgentDAO
pub struct AgentRepo {
    pool: PgPool,
}

impl AgentRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_all(&self) -> Result<Vec<Agent>, sqlx::Error> {
        let agents = sqlx::query_as!(
            Agent,
            r#"
            SELECT 
                id,
                key,
                name,
                status
            FROM agents
            ORDER BY id
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(agents)
    }
}
