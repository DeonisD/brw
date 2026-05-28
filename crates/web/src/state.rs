use sqlx::PgPool;

/// Общее состояние приложения, передаётся во все обработчики
/// через axum::extract::State
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

impl AppState {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}
