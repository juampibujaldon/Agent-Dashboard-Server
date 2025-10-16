use agent::models::system_metrics::SystemMetrics;
use agent::services::client::Client;
use agent::services::metrics_service::MetricsService;
use agent::services::system_monitor::SystemMonitor;
use mockito::{Matcher, Server};

struct FakeMonitor {
    metrics: SystemMetrics,
}

impl SystemMonitor for FakeMonitor {
    fn collect(&mut self) -> SystemMetrics {
        self.metrics.clone()
    }
}

#[tokio::test]
async fn metrics_service_uses_server_id() {
    let mut server = Server::new_async().await;

    let expected_body = serde_json::json!({
        "server_id": "test-srv",
        "cpu_usage": 10.0,
        "ram_usage": 20.0,
        "disk_space": 30.0,
        "temperature": 40.0
    });

    let _m = server
        .mock("POST", "/metrics")
        .match_header("x-api-key", "key")
        .match_body(Matcher::Json(expected_body))
        .with_status(200)
        .create();

    let monitor = FakeMonitor {
        metrics: SystemMetrics::new(10.0, 20.0, 30.0, 40.0),
    };
    let client = Client::new(server.url(), "key");
    let mut service = MetricsService::new(monitor, client, "test-srv");

    service
        .collect_and_publish()
        .await
        .expect("debería publicar sin errores");
}
