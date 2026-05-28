use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde_json::json;

/// Единый тип ошибок API
#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    Internal(String),
    Unauthorized(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, code, message) = match &self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, "404", msg.clone()),
            AppError::Internal(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "500", msg.clone())
            }
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, "401", msg.clone()),
        };

        let body = Json(json!({
            "code": code,
            "message": message
        }));

        (status, body).into_response()
    }
}

// Автоматическое преобразование ошибок из сервисного слоя
impl From<ports::agent_api::AgentError> for AppError {
    fn from(err: ports::agent_api::AgentError) -> Self {
        match err {
            ports::agent_api::AgentError::NotFound(msg) => AppError::NotFound(msg),
            ports::agent_api::AgentError::Database(msg) => AppError::Internal(msg),
        }
    }
}
