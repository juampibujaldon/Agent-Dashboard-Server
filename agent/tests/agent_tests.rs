
use agent::models::metrics::Metrica;
use agent::services::client::enviar_metricas;
use agent::services::metrics::recolectar_metricas;


#[test]
fn test_recolectar_metricas_devuelve_datos_validos() {
    let metrica = recolectar_metricas();

    assert!(metrica.cpu_usage >= 0.0 && metrica.cpu_usage <= 100.0, "El uso de CPU debe estar entre 0 y 100");
    assert!(metrica.ram_usage >= 0.0, "El uso de RAM debe ser un valor positivo");
    assert!(metrica.timestamp > 1672531200, "El timestamp parece ser inválido");
}


#[tokio::test]
async fn test_enviar_metricas_llama_al_backend_correctamente() {

    let mut server = mockito::Server::new_async().await;


    let mock = server.mock("POST", "/metrics")
        .with_status(200)
        .with_body("Metricas recibidas")
        .create_async().await;

    let metrica_de_prueba = Metrica {
        servidor_id: "test-server-01".to_string(),
        timestamp: 1672531200,
        cpu_usage: 42.5,
        ram_usage: 8192.0,
    };

    let url = server.url();


    let resultado = enviar_metricas(&url, &metrica_de_prueba).await;


    assert!(resultado.is_ok(), "La función de envío de métricas debería haber retornado Ok");
    mock.assert_async().await;
}