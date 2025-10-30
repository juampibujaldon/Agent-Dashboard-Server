use crate::services::client::Client;
use crate::services::fake_monitor::FakeMonitor;
use crate::models::system_metrics::SystemMetrics;
use std::collections::HashMap;
use tracing::{info, error};

#[derive(Debug, Clone)]
pub struct MultiServerResult {
    pub successful_servers: Vec<String>,
    pub failed_servers: HashMap<String, String>,
    pub total_servers: usize,
}

impl MultiServerResult {
    pub fn new() -> Self {
        Self {
            successful_servers: Vec::new(),
            failed_servers: HashMap::new(),
            total_servers: 0,
        }
    }

    pub fn add_success(&mut self, server_id: String) {
        self.successful_servers.push(server_id);
        self.total_servers += 1;
    }

    pub fn add_failure(&mut self, server_id: String, error: crate::AppError) {
        self.failed_servers.insert(server_id, error.to_string());
        self.total_servers += 1;
    }

    pub fn all_successful(&self) -> bool {
        self.failed_servers.is_empty()
    }

    pub fn any_successful(&self) -> bool {
        !self.successful_servers.is_empty()
    }

    pub fn success_rate(&self) -> f64 {
        if self.total_servers == 0 {
            0.0
        } else {
            (self.successful_servers.len() as f64 / self.total_servers as f64) * 100.0
        }
    }
}

impl Default for MultiServerResult {
    fn default() -> Self {
        Self::new()
    }
}


pub struct MultiServerMetricsService {
    client: Client,
    monitor: FakeMonitor,
}

impl MultiServerMetricsService {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            monitor: FakeMonitor::new(),
        }
    }

    
    pub fn with_servers(client: Client, servers: Vec<String>) -> Self {
        Self {
            client,
            monitor: FakeMonitor::with_servers(servers),
        }
    }

    
    pub fn get_servers(&self) -> Vec<String> {
        self.monitor.get_servers()
    }

    
    pub fn collect_all_metrics(&mut self) -> HashMap<String, SystemMetrics> {
        self.monitor.collect_all_metrics()
    }

    
    pub async fn collect_and_publish_all(&mut self) -> Result<MultiServerResult, crate::AppError> {
        let all_metrics = self.collect_all_metrics();
        let mut results = MultiServerResult::new();
        
        info!(
            server_count = all_metrics.len(),
            "Recolectando métricas para {} servidores",
            all_metrics.len()
        );

        for (server_id, metrics) in all_metrics {
            let payload = metrics.into_payload_with_timestamp(server_id.clone());
            
            match self.client.send_metric(&payload).await {
                Ok(_) => {
                    let timestamp_info = payload.formatted_timestamp()
                        .map(|ts| format!(" [{}]", ts))
                        .unwrap_or_default();
                    info!(
                        server_id = %server_id, 
                        timestamp = ?payload.timestamp,
                        "Métricas enviadas exitosamente{}", 
                        timestamp_info
                    );
                    results.add_success(server_id.clone());
                }
                Err(e) => {
                    error!(server_id = %server_id, error = %e, "Error enviando métricas");
                    results.add_failure(server_id, e);
                }
            }
        }

        Ok(results)
    }

    
    pub async fn collect_and_publish_server(&mut self, server_id: &str) -> Result<(), crate::AppError> {
        let all_metrics = self.collect_all_metrics();
        
        if let Some(metrics) = all_metrics.get(server_id) {
            let payload = metrics.clone().into_payload_with_timestamp(server_id);
            self.client.send_metric(&payload).await?;
            info!(server_id = %server_id, "Métricas enviadas para servidor específico");
        } else {
            return Err(crate::AppError::Validation(format!("Servidor {} no encontrado", server_id)));
        }

        Ok(())
    }
}

impl Default for MultiServerMetricsService {
    fn default() -> Self {
        Self::new(Client::new("http://localhost:5001/api", "not-needed").unwrap())
    }
}
