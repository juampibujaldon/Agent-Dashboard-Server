use agent::config::settings::Settings;
use agent::services::client::Client;
use agent::services::multi_server_service::MultiServerMetricsService;
use tracing::{error, info, warn};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let settings = Settings::load();
    info!(
        backend = %settings.backend_base_url,
        interval = settings.interval_secs,
        "Iniciando agente de monitoreo multi-servidor con métricas aleatorias"
    );

    let client = Client::from_settings(&settings);
    let mut service = MultiServerMetricsService::new(client);

    info!(
        server_count = service.get_servers().len(),
        servers = ?service.get_servers(),
        "Servidores configurados para monitoreo con fluctuación realista"
    );

    loop {
        match service.collect_and_publish_all().await {
            Ok(result) => {
                if result.all_successful() {
                    info!(
                        success_rate = result.success_rate(),
                        "Métricas aleatorias enviadas exitosamente para todos los servidores"
                    );
                } else {
                    warn!(
                        success_rate = result.success_rate(),
                        successful = result.successful_servers.len(),
                        failed = result.failed_servers.len(),
                        "Algunas métricas aleatorias fallaron al enviarse"
                    );
                    
                    // Log detallado de errores
                    for (server_id, error_msg) in &result.failed_servers {
                        error!(server_id = %server_id, error = %error_msg, "Error en servidor");
                    }
                }
            }
            Err(err) => {
                error!(error = %err, "Error crítico en el servicio de métricas aleatorias");
            }
        }
        
        tokio::time::sleep(std::time::Duration::from_secs(settings.interval_secs)).await;
    }
}
