
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metric {
    pub id: Option<String>,
    pub name: String,
    pub value: f64,
    pub unit: String,
    pub timestamp: DateTime<Utc>,
    pub server_id: String,
    pub category: MetricCategory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricCategory {
    CPU,
    Memory,
    Disk,
    Network,
    Custom(String),
}

impl Metric {
    pub fn new(
        name: impl Into<String>,
        value: f64,
        unit: impl Into<String>,
        server_id: impl Into<String>,
        category: MetricCategory,
    ) -> Self {
        Self {
            id: None,
            name: name.into(),
            value,
            unit: unit.into(),
            timestamp: Utc::now(),
            server_id: server_id.into(),
            category,
        }
    }
    
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }
    
    pub fn is_critical(&self) -> bool {
        match self.category {
            MetricCategory::CPU => self.value > 90.0,
            MetricCategory::Memory => self.value > 95.0,
            MetricCategory::Disk => self.value > 90.0,
            _ => false,
        }
    }
}

