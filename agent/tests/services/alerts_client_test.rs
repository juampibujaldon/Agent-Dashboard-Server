use agent::models::alert::{Alert, AlertCondition};
use agent::services::alerts_client::AlertsClient;
use agent::AppError;
use mockito::{Matcher, Server};
use tokio;

fn sample_alert() -> Alert {
    Alert::new("server_1", "cpu_usage", 90.0, AlertCondition::GreaterThan).unwrap()
}

#[tokio::test]
async fn send_alert_success_200() {
    let mut server = Server::new_async().await;
    let alert = sample_alert();
    let api_key = "k123";

    let _m = server
        .mock("POST", "/alerts")
        .match_header("x-api-key", api_key)
        .match_header("content-type", Matcher::Regex("application/json".into()))
        .match_body(Matcher::JsonString(serde_json::to_string(&alert).unwrap()))
        .with_status(200)
        .create();

    let client = AlertsClient::new(server.url(), api_key).unwrap();
    assert!(client.send_alert(&alert).await.is_ok());
}

#[tokio::test]
async fn send_alert_validation_fails_locally() {
    let server = Server::new_async().await;
    let mut alert = sample_alert();
    alert.metric_type = "".into();

    let client = AlertsClient::new(server.url(), "k123").unwrap();
    let err = client.send_alert(&alert).await.unwrap_err();
    matches!(err, AppError::Validation(_));
}

#[tokio::test]
async fn send_alert_server_error_500_with_retries() {
    let mut server = Server::new_async().await;
    let alert = sample_alert();
    let api_key = "k123";

    let _m = server
        .mock("POST", "/alerts")
        .match_header("x-api-key", api_key)
        .with_status(500)
        .expect_at_least(2) 
        .create();

    let client = AlertsClient::new(server.url(), api_key).unwrap();
    let result = client.send_alert(&alert).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn send_alert_timeout_with_retries() {
    let alert = sample_alert();
    let api_key = "k123";

    let client = AlertsClient::new("http://invalid-host:9999", api_key).unwrap();
    let result = client.send_alert(&alert).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn send_alert_empty_server_id() {
    let server = Server::new_async().await;
    let mut alert = sample_alert();
    alert.server_id = "".into();

    let client = AlertsClient::new(server.url(), "k123").unwrap();
    let err = client.send_alert(&alert).await.unwrap_err();
    assert!(matches!(err, AppError::Validation(_)));
}