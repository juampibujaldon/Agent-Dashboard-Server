
use crate::models::metrics::Metrica;

pub async fn enviar_metricas(api_endpoint: &str, metrica: &Metrica) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();

    let url = format!("{}/metrics", api_endpoint);

    println!("Enviando métricas a: {}", url);

    client.post(&url)
        .json(metrica)
        .send()
        .await? 
        .error_for_status()?; 

    println!("Métricas enviadas con éxito.");
    Ok(())
}