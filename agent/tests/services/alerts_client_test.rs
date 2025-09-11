use agent::models::alert::{Alert, AlertCondition};
use agent::services::alerts_client::AlertsClient;
use agent::AppError;
use mockito::{Matcher, Server};
use tokio;

fn sample_alert() -> Alert {
    Alert::new("server_1", "cpu_usage", 90.0, AlertCondition::GreaterThan)
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

    let client = AlertsClient::new(server.url(), api_key);
    assert!(client.send_alert(&alert).await.is_ok());
}

#[tokio::test]
async fn send_alert_validation_fails_locally() {
    let server = Server::new_async().await;
    let mut alert = sample_alert();
    alert.metric_type = "".into();

    let client = AlertsClient::new(server.url(), "k123");
    let err = client.send_alert(&alert).await.unwrap_err();
    matches!(err, AppError::Validation(_));
}
