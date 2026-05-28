use serde::{Deserialize, Serialize};

/// Агент — соответствует com.itrail.domain.Agent (record)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: Option<i64>,
    pub key: String,
    pub name: String,
    pub status: Option<i16>,
}
