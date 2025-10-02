use crate::models::payloads::MetricPayload;

#[derive(Debug, Clone, Default)]
pub struct SystemMetrics {
    pub cpu_usage: f32,
    pub ram_usage: f32,
    pub disk_space: f32,
    pub temperature: f32,
}

impl SystemMetrics {
    pub fn new(cpu_usage: f32, ram_usage: f32, disk_space: f32, temperature: f32) -> Self {
        Self {
            cpu_usage,
            ram_usage,
            disk_space,
            temperature,
        }
    }

    pub fn into_payload(self, server_id: impl Into<String>) -> MetricPayload {
        MetricPayload::new(
            server_id,
            self.cpu_usage,
            self.ram_usage,
            self.disk_space,
            self.temperature,
        )
    }
}
