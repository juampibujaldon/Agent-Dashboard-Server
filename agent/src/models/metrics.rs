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
    ) -> crate::Result<Self> {
        let name = name.into();
        let unit = unit.into();
        let server_id = server_id.into();
        
        if name.trim().is_empty() {
            return Err(crate::AppError::Validation("Metric name cannot be empty".into()));
        }
        
        if unit.trim().is_empty() {
            return Err(crate::AppError::Validation("Metric unit cannot be empty".into()));
        }
        
        if server_id.trim().is_empty() {
            return Err(crate::AppError::Validation("Server ID cannot be empty".into()));
        }
        
        if value.is_nan() {
            return Err(crate::AppError::Validation("Metric value cannot be NaN".into()));
        }
        
        if value.is_infinite() {
            return Err(crate::AppError::Validation("Metric value cannot be infinite".into()));
        }
        
        if value < 0.0 {
            return Err(crate::AppError::Validation("Metric value cannot be negative".into()));
        }
        
        Ok(Self {
            id: None,
            name,
            value,
            unit,
            timestamp: Utc::now(),
            server_id,
            category,
        })
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