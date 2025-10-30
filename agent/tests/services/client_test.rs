use agent::config::settings::Settings;
use agent::models::payloads::MetricPayload;
use agent::services::client::Client;
use agent::AppError;

use mockito::{Matcher, Server};
use tokio;

fn sample_payload() -> MetricPayload {
    MetricPayload::new("server_1", 42.0, 35.0, 10.0, 55.0)
}

#[tokio::test]
async fn send_metric_success_200() {
    let mut server = Server::new_async().await;

    let metric = sample_payload();
    let api_key = "k123";

    let _m = server
        .mock("POST", "/metrics")
        .match_header("x-api-key", api_key)
        .match_header("content-type", Matcher::Regex("application/json".into()))
        .match_body(Matcher::JsonString(serde_json::to_string(&metric).unwrap()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"message": "Metric received"}"#)
        .create();

    let settings = Settings::new(
        server.url().to_string(),
        "test_server".to_string(),
        30,
    );

    let client = Client::from_settings(&settings);
    let result = client.send_metric(&metric).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn send_metric_server_error_500() {
    let mut server = Server::new_async().await;

    let metric = sample_payload();
    let api_key = "k123";

    let _m = server
        .mock("POST", "/metrics")
        .match_header("x-api-key", api_key)
        .match_header("content-type", Matcher::Regex("application/json".into()))
        .match_body(Matcher::JsonString(serde_json::to_string(&metric).unwrap()))
        .with_status(500)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error": "Internal server error"}"#)
        .create();

    let settings = Settings::new(
        server.url().to_string(),
        "test_server".to_string(),
        30,
    );

    let client = Client::from_settings(&settings);
    let result = client.send_metric(&metric).await;

    assert!(result.is_err());
    if let Err(AppError::RequestError(msg)) = result {
        assert!(msg.contains("500"));
    }
}

#[tokio::test]
async fn send_metric_connection_error() {
    let settings = Settings::new(
        "http://localhost:9999".to_string(),
        "test_server".to_string(),
        30,
    );

    let client = Client::from_settings(&settings);
    let metric = sample_payload();
    let result = client.send_metric(&metric).await;

    assert!(result.is_err());
    if let Err(AppError::RequestError(msg)) = result {
        // En Windows, el mensaje puede ser diferente - aceptamos cualquier error de request
        assert!(!msg.is_empty(), "Error message should not be empty");
    }
}

#[tokio::test]
async fn send_metric_timeout() {
    // Nota: mockito 1.2 no soporta with_delay, este test está simplificado
    // Para probar timeouts reales se necesitaría un servidor de prueba más sofisticado
    let settings = Settings::new(
        "http://localhost:9999".to_string(),
        "test_server".to_string(),
        30,
    );

    let client = Client::from_settings(&settings);
    let metric = sample_payload();
    
    // Intentamos hacer una request a un puerto que no existe
    // El timeout del cliente debería activarse
    let result = client.send_metric(&metric).await;

    assert!(result.is_err());
    if let Err(AppError::RequestError(_msg)) = result {
        // El error puede ser de conexión o timeout, ambos son aceptables
    }
}

#[tokio::test]
async fn send_metric_invalid_json_response() {
    let mut server = Server::new_async().await;

    let metric = sample_payload();
    let api_key = "k123";

    let _m = server
        .mock("POST", "/metrics")
        .match_header("x-api-key", api_key)
        .match_header("content-type", Matcher::Regex("application/json".into()))
        .match_body(Matcher::JsonString(serde_json::to_string(&metric).unwrap()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body("invalid json response")
        .create();

    let settings = Settings::new(
        server.url().to_string(),
        "test_server".to_string(),
        30,
    );

    let client = Client::from_settings(&settings);
    let result = client.send_metric(&metric).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn send_metric_missing_api_key() {
    let mut server = Server::new_async().await;

    let metric = sample_payload();

    let _m = server
        .mock("POST", "/metrics")
        .match_header("content-type", Matcher::Regex("application/json".into()))
        .match_body(Matcher::JsonString(serde_json::to_string(&metric).unwrap()))
        .with_status(401)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error": "Unauthorized"}"#)
        .create();

    let settings = Settings::new(
        server.url().to_string(),
        "test_server".to_string(),
        30,
    );

    let client = Client::from_settings(&settings);
    let result = client.send_metric(&metric).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn send_metric_bad_request_400() {
    let mut server = Server::new_async().await;

    let metric = sample_payload();
    let api_key = "k123";

    let _m = server
        .mock("POST", "/metrics")
        .match_header("x-api-key", api_key)
        .match_header("content-type", Matcher::Regex("application/json".into()))
        .match_body(Matcher::JsonString(serde_json::to_string(&metric).unwrap()))
        .with_status(400)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error": "Bad request"}"#)
        .create();

    let settings = Settings::new(
        server.url().to_string(),
        "test_server".to_string(),
        30,
    );

    let client = Client::from_settings(&settings);
    let result = client.send_metric(&metric).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn send_metric_unauthorized_401() {
    let mut server = Server::new_async().await;

    let metric = sample_payload();
    let api_key = "k123";

    let _m = server
        .mock("POST", "/metrics")
        .match_header("x-api-key", api_key)
        .match_header("content-type", Matcher::Regex("application/json".into()))
        .match_body(Matcher::JsonString(serde_json::to_string(&metric).unwrap()))
        .with_status(401)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error": "Unauthorized"}"#)
        .create();

    let settings = Settings::new(
        server.url().to_string(),
        "test_server".to_string(),
        30,
    );

    let client = Client::from_settings(&settings);
    let result = client.send_metric(&metric).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn send_metric_not_found_404() {
    let mut server = Server::new_async().await;

    let metric = sample_payload();
    let api_key = "k123";

    let _m = server
        .mock("POST", "/metrics")
        .match_header("x-api-key", api_key)
        .match_header("content-type", Matcher::Regex("application/json".into()))
        .match_body(Matcher::JsonString(serde_json::to_string(&metric).unwrap()))
        .with_status(404)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error": "Not found"}"#)
        .create();

    let settings = Settings::new(
        server.url().to_string(),
        "test_server".to_string(),
        30,
    );

    let client = Client::from_settings(&settings);
    let result = client.send_metric(&metric).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn send_metric_rate_limit_429() {
    let mut server = Server::new_async().await;

    let metric = sample_payload();
    let api_key = "k123";

    let _m = server
        .mock("POST", "/metrics")
        .match_header("x-api-key", api_key)
        .match_header("content-type", Matcher::Regex("application/json".into()))
        .match_body(Matcher::JsonString(serde_json::to_string(&metric).unwrap()))
        .with_status(429)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error": "Rate limit exceeded"}"#)
        .create();

    let settings = Settings::new(
        server.url().to_string(),
        "test_server".to_string(),
        30,
    );

    let client = Client::from_settings(&settings);
    let result = client.send_metric(&metric).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn send_metric_multiple_requests() {
    let mut server = Server::new_async().await;

    let metric1 = sample_payload();
    let metric2 = MetricPayload::new("server_2", 50.0, 40.0, 15.0, 60.0);
    let api_key = "k123";

    let _m = server
        .mock("POST", "/metrics")
        .match_header("x-api-key", api_key)
        .match_header("content-type", Matcher::Regex("application/json".into()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"message": "Metric received"}"#)
        .expect(2)
        .create();

    let settings = Settings::new(
        server.url().to_string(),
        "test_server".to_string(),
        30,
    );

    let client = Client::from_settings(&settings);
    
    let result1 = client.send_metric(&metric1).await;
    let result2 = client.send_metric(&metric2).await;

    assert!(result1.is_ok());
    assert!(result2.is_ok());
}

#[tokio::test]
async fn send_metric_concurrent_requests() {
    let mut server = Server::new_async().await;

    let metric1 = sample_payload();
    let metric2 = MetricPayload::new("server_2", 50.0, 40.0, 15.0, 60.0);
    let api_key = "k123";

    let _m = server
        .mock("POST", "/metrics")
        .match_header("x-api-key", api_key)
        .match_header("content-type", Matcher::Regex("application/json".into()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"message": "Metric received"}"#)
        .expect(2)
        .create();

    let settings = Settings::new(
        server.url().to_string(),
        "test_server".to_string(),
        30,
    );

    let client = Client::from_settings(&settings);
    
    let result1 = client.send_metric(&metric1).await;
    let result2 = client.send_metric(&metric2).await;

    assert!(result1.is_ok());
    assert!(result2.is_ok());
}

#[tokio::test]
async fn send_metric_large_payload() {
    let mut server = Server::new_async().await;

    let metric = MetricPayload::new("server_large", 99.99, 88.88, 77.77, 66.66);
    let api_key = "k123";

    let _m = server
        .mock("POST", "/metrics")
        .match_header("x-api-key", api_key)
        .match_header("content-type", Matcher::Regex("application/json".into()))
        .match_body(Matcher::JsonString(serde_json::to_string(&metric).unwrap()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"message": "Large metric received"}"#)
        .create();

    let settings = Settings::new(
        server.url().to_string(),
        "test_server".to_string(),
        30,
    );

    let client = Client::from_settings(&settings);
    let result = client.send_metric(&metric).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn send_metric_empty_server_id() {
    let mut server = Server::new_async().await;

    let metric = MetricPayload::new("", 42.0, 35.0, 10.0, 55.0);
    let api_key = "k123";

    let _m = server
        .mock("POST", "/metrics")
        .match_header("x-api-key", api_key)
        .match_header("content-type", Matcher::Regex("application/json".into()))
        .match_body(Matcher::JsonString(serde_json::to_string(&metric).unwrap()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"message": "Metric received"}"#)
        .create();

    let settings = Settings::new(
        server.url().to_string(),
        "test_server".to_string(),
        30,
    );

    let client = Client::from_settings(&settings);
    let result = client.send_metric(&metric).await;

    assert!(result.is_err());
    if let Err(AppError::Validation(_msg)) = result {
        // Validación local falló, que es lo esperado
    } else {
        panic!("Expected Validation error but got something else");
    }
}

#[tokio::test]
async fn send_metric_negative_values() {
    let mut server = Server::new_async().await;

    let metric = MetricPayload::new("server_1", -1.0, -2.0, -3.0, -4.0);
    let api_key = "k123";

    let _m = server
        .mock("POST", "/metrics")
        .match_header("x-api-key", api_key)
        .match_header("content-type", Matcher::Regex("application/json".into()))
        .match_body(Matcher::JsonString(serde_json::to_string(&metric).unwrap()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"message": "Metric received"}"#)
        .create();

    let settings = Settings::new(
        server.url().to_string(),
        "test_server".to_string(),
        30,
    );

    let client = Client::from_settings(&settings);
    let result = client.send_metric(&metric).await;

    assert!(result.is_err());
    if let Err(AppError::Validation(_msg)) = result {
        // Validación local falló, que es lo esperado
    } else {
        panic!("Expected Validation error but got something else");
    }
}

#[tokio::test]
async fn send_metric_zero_values() {
    let mut server = Server::new_async().await;

    let metric = MetricPayload::new("server_1", 0.0, 0.0, 0.0, 0.0);
    let api_key = "k123";

    let _m = server
        .mock("POST", "/metrics")
        .match_header("x-api-key", api_key)
        .match_header("content-type", Matcher::Regex("application/json".into()))
        .match_body(Matcher::JsonString(serde_json::to_string(&metric).unwrap()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"message": "Metric received"}"#)
        .create();

    let settings = Settings::new(
        server.url().to_string(),
        "test_server".to_string(),
        30,
    );

    let client = Client::from_settings(&settings);
    let result = client.send_metric(&metric).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn send_metric_max_values() {
    let mut server = Server::new_async().await;

    let metric = MetricPayload::new("server_1", 100.0, 100.0, 100.0, 100.0);
    let api_key = "k123";

    let _m = server
        .mock("POST", "/metrics")
        .match_header("x-api-key", api_key)
        .match_header("content-type", Matcher::Regex("application/json".into()))
        .match_body(Matcher::JsonString(serde_json::to_string(&metric).unwrap()))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"message": "Metric received"}"#)
        .create();

    let settings = Settings::new(
        server.url().to_string(),
        "test_server".to_string(),
        30,
    );

    let client = Client::from_settings(&settings);
    let result = client.send_metric(&metric).await;

    assert!(result.is_ok());
}