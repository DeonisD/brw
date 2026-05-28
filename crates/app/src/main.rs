use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Инициализация логирования
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "rwetd=debug,tower_http=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Загрузка конфигурации
    let config = config::Config::from_env();

    tracing::info!("Подключение к базе данных...");
    let pool = db::create_pool(&config.database_url).await?;

    // Создаём состояние приложения
    let state = web::state::AppState::new(pool);

    // Собираем роутер
    let app = web::router::create_router(state);

    // Запускаем сервер
    let addr = SocketAddr::new(config.server_host, config.server_port);
    tracing::info!("Сервер запущен на http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
