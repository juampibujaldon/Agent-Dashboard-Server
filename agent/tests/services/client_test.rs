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
        .create();

    let base_url = server.url();
    let client = Client::new(base_url, api_key);

    let res = client.send_metric(&metric).await;
    assert!(res.is_ok(), "send_metric debería terminar OK con 200");
}

#[tokio::test]
async fn send_metric_client_error_400_returns_err_without_retry() {
    let mut server = Server::new_async().await;

    let metric = sample_payload();
    let api_key = "k123";

    let _m = server
        .mock("POST", "/metrics")
        .match_header("x-api-key", api_key)
        .match_body(Matcher::JsonString(serde_json::to_string(&metric).unwrap()))
        .with_status(400)
        .with_body("bad request")
        .create();

    let base_url = server.url();
    let client = Client::new(base_url, api_key);

    let err = client.send_metric(&metric).await.unwrap_err();
    match err {
        AppError::Metrics(msg) => assert!(
            msg.contains("400"),
            "El mensaje debería mencionar 400, fue: {msg}"
        ),
        other => panic!("Se esperaba AppError::Metrics, obtuve: {other:?}"),
    }
}

#[tokio::test]
async fn send_metric_server_error_500_retries_and_fails() {
    let mut server = Server::new_async().await;

    let metric = sample_payload();
    let api_key = "k123";

    let _m = server
        .mock("POST", "/metrics")
        .match_header("x-api-key", api_key)
        .match_body(Matcher::JsonString(serde_json::to_string(&metric).unwrap()))
        .with_status(500)
        .expect(3)
        .create();

    let base_url = server.url();
    let client = Client::new(base_url, api_key);

    let err = client.send_metric(&metric).await.unwrap_err();
    match err {
        AppError::Metrics(msg) => assert!(
            msg.contains("after"),
            "Debería indicar que falló tras reintentos: {msg}"
        ),
        other => panic!("Se esperaba AppError::Metrics, obtuve: {other:?}"),
    }
}

#[tokio::test]
async fn health_check_ok_200() {
    let mut server = Server::new_async().await;

    let api_key = "k123";

    let _m = server
        .mock("GET", "/health")
        .match_header("x-api-key", api_key)
        .with_status(200)
        .create();

    let base_url = server.url();
    let client = Client::new(base_url, api_key);

    let res = client.health_check().await;
    assert!(res.is_ok());
}

#[tokio::test]
async fn health_check_fail_503() {
    let mut server = Server::new_async().await;

    let api_key = "k123";

    let _m = server
        .mock("GET", "/health")
        .match_header("x-api-key", api_key)
        .with_status(503)
        .create();

    let base_url = server.url();
    let client = Client::new(base_url, api_key);

    let err = client.health_check().await.unwrap_err();
    matches!(err, AppError::Metrics(_));
}

#[tokio::test]
async fn send_metrics_batch_counts_ok() {
    let mut server = Server::new_async().await;

    let m1 = sample_payload();
    let m2 = MetricPayload::new("server_1", 65.0, 55.0, 30.0, 40.0);

    let api_key = "k123";
    let base_url = server.url();
    let client = Client::new(base_url, api_key);

    let _p1 = server
        .mock("POST", "/metrics")
        .match_header("x-api-key", api_key)
        .match_body(Matcher::JsonString(serde_json::to_string(&m1).unwrap()))
        .with_status(200)
        .create();

    let _p2 = server
        .mock("POST", "/metrics")
        .match_header("x-api-key", api_key)
        .match_body(Matcher::JsonString(serde_json::to_string(&m2).unwrap()))
        .with_status(200)
        .create();

    let count = client.send_metrics_batch(&[m1, m2]).await.unwrap();
    assert_eq!(count, 2);
}

#[tokio::test]
async fn send_metric_validation_fails_locally_no_http_call() {
    let server = Server::new_async().await;

    let mut invalid = sample_payload();
    invalid.disk_space = 150.0;

    let api_key = "k123";
    let base_url = server.url();
    let client = Client::new(base_url, api_key);

    let err = client.send_metric(&invalid).await.unwrap_err();
    matches!(err, AppError::Validation(_));
}

#[tokio::test]
async fn from_settings_constructor_works() {
    let mut server = Server::new_async().await;

    let mut settings = Settings::default();
    settings.api_key = "abc".into();
    settings.backend_base_url = server.url();
    let client = Client::from_settings(&settings);

    let _m = server
        .mock("GET", "/health")
        .match_header("x-api-key", "abc")
        .with_status(200)
        .create();

    assert!(client.health_check().await.is_ok());
}
