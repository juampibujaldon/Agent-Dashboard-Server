use agent::services::{client::enviar_metricas, metrics::recolectar_metricas};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    println!("Iniciando agente de monitoreo...");

    // TODO: Cargar esto desde una variable de entorno o archivo de configuración.
    let backend_url = "http://localhost:8000";

    loop {
        println!("\nRecolectando métricas...");
        let metrica = recolectar_metricas();
        println!("Métricas recolectadas: {:?}", metrica);

        if let Err(e) = enviar_metricas(backend_url, &metrica).await {
            eprintln!("Error al enviar métricas: {}", e);
        }
        
        sleep(Duration::from_secs(10)).await;
    }
}

