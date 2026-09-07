#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use paugeran::{
        config::AppConfig,
        database::Database,
        http::{app::create_app, state::AppState},
    };
    tracing_subscriber::fmt()
        .with_env_filter(std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()))
        .init();
    let config = AppConfig::from_env();
    let database = Database::connect(config.data_dir.to_str().unwrap_or("./data")).await?;
    database.seed_if_empty().await?;
    let app = create_app(AppState {
        started_at: chrono::Utc::now(),
        database,
        pii_redactor: paugeran::crypto::pii_redactor::PiiRedactor::default(),
    });
    let address = config.address()?;
    tracing::info!(%address, "PAUGERAN server listening");
    let listener = tokio::net::TcpListener::bind(address).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
