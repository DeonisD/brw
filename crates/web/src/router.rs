use axum::{middleware, Router};

use crate::{
    middleware::auth::require_auth,
    routes::agent_routes,
    state::AppState,
};

/// Создаёт роутер со всеми маршрутами приложения
pub fn create_router(state: AppState) -> Router {
    // Публичные маршруты (без аутентификации)
    let public_routes = Router::new();

    // Защищённые маршруты (требуют Bearer токен)
    let protected_routes = Router::new()
        .route("/agents", axum::routing::get(agent_routes::get_agents))
        // Здесь позже добавим остальные маршруты:
        // .route("/orders/{id}", get(order_routes::get_order))
        .route_layer(middleware::from_fn(require_auth));

    // Объединяем
    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .with_state(state)
}
