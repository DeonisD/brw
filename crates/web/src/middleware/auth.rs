use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};

/// Middleware для проверки Bearer токена
/// Аналог @AgentRequired + AgentFilter
pub async fn require_auth(
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "));

    match auth_header {
        Some(token) => {
            tracing::debug!("Токен получен: {}...", &token[..token.len().min(10)]);
            // TODO: здесь будет твоя логика проверки JWT
            // Пока пропускаем любой непустой токен
            Ok(next.run(req).await)
        }
        None => {
            tracing::warn!("Отсутствует Authorization заголовок");
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}
