use crate::models::system_metrics::SystemMetrics;
use crate::services::client::Client;
use crate::traits::monitor::SystemMonitor;
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

    pub fn collect_metrics(&mut self) -> SystemMetrics {
        self.monitor.collect()
    }

    pub async fn collect_and_publish(&mut self) -> Result<()> {
        let metrics = self.collect_metrics();
        let payload = metrics.into_payload_with_timestamp(self.server_id.clone());
        self.client.send_metric(&payload).await
    }

    pub fn get_servers(&self) -> Vec<String> {
        self.monitor.get_servers()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::monitor::SystemMonitor;
    use mockito::Server;

    struct StubMonitor {
        metrics: SystemMetrics,
    }

    impl SystemMonitor for StubMonitor {
        fn collect(&mut self) -> SystemMetrics {
            self.metrics.clone()
        }

        fn get_servers(&self) -> Vec<String> {
            vec!["test-server".to_string()]
        }
    }

    #[tokio::test]
    async fn collect_and_publish_sends_payload() {
        let mut server = Server::new_async().await;

        let _m = server
            .mock("POST", "/metrics")
            .match_header("x-api-key", "key")
            .with_status(200)
            .create();

        let client = Client::new(server.url(), "key").unwrap();
        let monitor = StubMonitor {
            metrics: SystemMetrics::new(40.0, 50.0, 60.0, 35.0),
        };
        let mut service = MetricsService::new(monitor, client, "s1");

        service.collect_and_publish().await.unwrap();
    }
}
