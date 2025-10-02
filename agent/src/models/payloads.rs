use serde::Serialize;

/// Representa el payload esperado por el backend Flask.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MetricPayload {
    #[serde(rename = "serverId")]
    pub server_id: String,
    pub cpu_usage: f32,
    pub ram_usage: f32,
    pub disk_space: f32,
    pub temperature: f32,
}

impl MetricPayload {
    pub fn new(
        server_id: impl Into<String>,
        cpu_usage: f32,
        ram_usage: f32,
        disk_space: f32,
        temperature: f32,
    ) -> Self {
        Self {
            server_id: server_id.into(),
            cpu_usage,
            ram_usage,
            disk_space,
            temperature,
        }
    }

    /// Valida los rangos básicos antes de enviar al backend.
    pub fn validate(&self) -> Result<(), String> {
        if self.server_id.trim().is_empty() {
            return Err("server_id vacío".into());
        }

        for (name, value) in [
            ("cpu_usage", self.cpu_usage),
            ("ram_usage", self.ram_usage),
            ("disk_space", self.disk_space),
        ] {
            if !(0.0..=100.0).contains(&value) {
                return Err(format!("{name} debe estar entre 0 y 100"));
            }
        }

        if !self.cpu_usage.is_finite()
            || !self.ram_usage.is_finite()
            || !self.disk_space.is_finite()
            || !self.temperature.is_finite()
        {
            return Err("Los valores no pueden ser NaN o infinitos".into());
        }

        Ok(())
    }
}
