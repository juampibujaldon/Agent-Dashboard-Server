use agent::config::settings::Settings;
use agent::services::client::Client;
use agent::services::metrics_service::MetricsService;
use agent::services::system_monitor::SysinfoMonitor;
use tracing::{error, info};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let settings = Settings::load();
    info!(
        server_id = %settings.server_id,
        backend = %settings.backend_base_url,
        "Iniciando agente"
    );

    let client = Client::from_settings(&settings);
    let monitor = SysinfoMonitor::new();
    let mut service = MetricsService::new(monitor, client, settings.server_id.clone());

    loop {
        if let Err(err) = service.collect_and_publish().await {
            error!("Error enviando métricas: {err}");
        }
        tokio::time::sleep(std::time::Duration::from_secs(settings.interval_secs)).await;
    }
}
