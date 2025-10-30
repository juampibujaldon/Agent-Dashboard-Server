use agent::models::system_metrics::SystemMetrics;
use agent::services::client::Client;
use agent::services::metrics_service::MetricsService;
use agent::traits::monitor::SystemMonitor;
use mockito::{Matcher, Server};


struct FakeMonitor {
    metrics: SystemMetrics,
}

impl SystemMonitor for FakeMonitor {
    fn collect(&mut self) -> SystemMetrics {
        self.metrics.clone()
    }

    fn get_servers(&self) -> Vec<String> {
        vec!["test-srv".to_string()]
    }
}

#[tokio::test]
async fn metrics_service_uses_server_id() {
    let mut server = Server::new_async().await;

    let _m = server
        .mock("POST", "/metrics")
        .match_header("x-api-key", "key")
        .match_header("content-type", Matcher::Regex("application/json".into()))
        .with_status(200)
        .create();

    let monitor = FakeMonitor {
        metrics: SystemMetrics::new(10.0, 20.0, 30.0, 40.0),
    };
    let client = Client::new(server.url(), "key").unwrap();
    let mut service = MetricsService::new(monitor, client, "test-srv");

    service
        .collect_and_publish()
        .await
        .expect("debería publicar sin errores");
}
