use crate::services::client::Client;
use crate::services::fake_monitor::FakeMonitor;
use crate::models::system_metrics::SystemMetrics;
use std::collections::HashMap;
use tracing::{info, error};

/// Resultado de operaciones multi-servidor
/// Implementa el principio SOLID de Responsabilidad Única
#[derive(Debug, Clone)]
pub struct MultiServerResult {
    pub successful_servers: Vec<String>,
    pub failed_servers: HashMap<String, String>,
    pub total_servers: usize,
}

impl MultiServerResult {
    /// Crea un nuevo resultado
    pub fn new() -> Self {
        Self {
            successful_servers: Vec::new(),
            failed_servers: HashMap::new(),
            total_servers: 0,
        }
    }

    /// Agrega un servidor exitoso
    pub fn add_success(&mut self, server_id: String) {
        self.successful_servers.push(server_id);
        self.total_servers += 1;
    }

    /// Agrega un servidor fallido
    pub fn add_failure(&mut self, server_id: String, error: crate::AppError) {
        self.failed_servers.insert(server_id, error.to_string());
        self.total_servers += 1;
    }

    /// Verifica si todos los servidores fueron exitosos
    pub fn all_successful(&self) -> bool {
        self.failed_servers.is_empty()
    }

    /// Verifica si algún servidor fue exitoso
    pub fn any_successful(&self) -> bool {
        !self.successful_servers.is_empty()
    }

    /// Calcula la tasa de éxito
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

/// Servicio para manejar métricas de múltiples servidores
/// Implementa el principio SOLID de Responsabilidad Única
pub struct MultiServerMetricsService {
    client: Client,
    monitor: FakeMonitor,
}

impl MultiServerMetricsService {
    /// Crea un nuevo servicio con servidores predefinidos
    pub fn new(client: Client) -> Self {
        Self {
            client,
            monitor: FakeMonitor::new(),
        }
    }

    /// Crea un servicio con servidores personalizados
    /// Implementa el principio SOLID de Abierto/Cerrado
    pub fn with_servers(client: Client, servers: Vec<String>) -> Self {
        Self {
            client,
            monitor: FakeMonitor::with_servers(servers),
        }
    }

    /// Obtiene la lista de servidores
    pub fn get_servers(&self) -> &Vec<String> {
        self.monitor.get_servers()
    }

    /// Recolecta métricas de todos los servidores
    /// Implementa operación CRUD - Read para múltiples entidades
    pub fn collect_all_metrics(&mut self) -> HashMap<String, SystemMetrics> {
        self.monitor.collect_all_metrics()
    }

    /// Recolecta y publica métricas de todos los servidores
    /// Implementa operación CRUD - Create para múltiples entidades
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

    /// Publica métricas de un servidor específico
    /// Implementa operación CRUD - Create para entidad específica
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
        Self::new(Client::new("http://localhost:5000/api", "not-needed").unwrap())
    }
}
