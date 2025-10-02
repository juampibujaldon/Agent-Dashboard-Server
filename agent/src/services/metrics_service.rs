use crate::models::payloads::MetricPayload;
use crate::models::system_metrics::SystemMetrics;
use crate::services::client::Client;
use crate::services::system_monitor::SystemMonitor;
use crate::Result;

pub struct MetricsService<M: SystemMonitor> {
    monitor: M,
    client: Client,
    server_id: String,
}

impl<M> MetricsService<M>
where
    M: SystemMonitor,
{
    pub fn new(monitor: M, client: Client, server_id: impl Into<String>) -> Self {
        Self {
            monitor,
            client,
            server_id: server_id.into(),
        }
    }

    pub fn snapshot(&mut self) -> SystemMetrics {
        self.monitor.collect()
    }

    pub async fn collect_and_publish(&mut self) -> Result<()> {
        let snapshot = self.snapshot();
        let payload = snapshot.into_payload(self.server_id.clone());
        self.send_payload(payload).await
    }

    async fn send_payload(&self, payload: MetricPayload) -> Result<()> {
        self.client.send_metric(&payload).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::system_monitor::SystemMonitor;
    use mockito::{Matcher, Server};

    struct StubMonitor {
        metrics: SystemMetrics,
    }

    impl SystemMonitor for StubMonitor {
        fn collect(&mut self) -> SystemMetrics {
            self.metrics.clone()
        }
    }

    #[tokio::test]
    async fn collect_and_publish_sends_payload() {
        let mut server = Server::new_async().await;

        let payload = MetricPayload::new("s1", 40.0, 50.0, 60.0, 35.0);
        let _m = server
            .mock("POST", "/metrics")
            .match_header("x-api-key", "key")
            .match_body(Matcher::JsonString(
                serde_json::to_string(&payload).unwrap(),
            ))
            .with_status(200)
            .create();

        let client = Client::new(server.url(), "key");
        let monitor = StubMonitor {
            metrics: SystemMetrics::new(40.0, 50.0, 60.0, 35.0),
        };
        let mut service = MetricsService::new(monitor, client, "s1");

        service.collect_and_publish().await.unwrap();
    }
}
