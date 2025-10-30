use serde::Serialize;
use chrono::{DateTime, FixedOffset};


#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MetricPayload {
    #[serde(rename = "server_id")]
    pub server_id: String,
    pub cpu_usage: f32,
    pub ram_usage: f32,
    pub disk_space: f32,
    pub temperature: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<FixedOffset>>,
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
            timestamp: None,
        }
    }

        pub fn new_with_timestamp(
        server_id: impl Into<String>,
        cpu_usage: f32,
        ram_usage: f32,
        disk_space: f32,
        temperature: f32,
    ) -> Self {
        let argentina_offset = FixedOffset::west_opt(3 * 3600).unwrap(); // UTC-3
        let now_argentina = chrono::Utc::now().with_timezone(&argentina_offset);
        
        Self {
            server_id: server_id.into(),
            cpu_usage,
            ram_usage,
            disk_space,
            temperature,
            timestamp: Some(now_argentina),
        }
    }

    
    pub fn formatted_timestamp(&self) -> Option<String> {
        self.timestamp.map(|ts| ts.format("%Y-%m-%d %H:%M:%S %z").to_string())
    }

    
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
