use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AlertCondition {
    GreaterThan,
    GreaterOrEqual,
    LessThan,
    LessOrEqual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub id: Option<String>,
    pub server_id: String,
    pub metric_type: String, 
    pub threshold: f64,
    pub condition: AlertCondition,
    pub created_at: DateTime<Utc>,
    pub is_active: bool,
}

impl Alert {
    pub fn new(
        server_id: impl Into<String>,
        metric_type: impl Into<String>,
        threshold: f64,
        condition: AlertCondition,
    ) -> crate::Result<Self> {
        let server_id = server_id.into();
        let metric_type = metric_type.into();
        
        if server_id.trim().is_empty() {
            return Err(crate::AppError::Validation("Server ID cannot be empty".into()));
        }
        
        if metric_type.trim().is_empty() {
            return Err(crate::AppError::Validation("Metric type cannot be empty".into()));
        }
        
        if threshold.is_nan() {
            return Err(crate::AppError::Validation("Threshold cannot be NaN".into()));
        }
        
        if threshold.is_infinite() {
            return Err(crate::AppError::Validation("Threshold cannot be infinite".into()));
        }
        
        if threshold < 0.0 {
            return Err(crate::AppError::Validation("Threshold cannot be negative".into()));
        }
        
        Ok(Self {
            id: None,
            server_id,
            metric_type,
            threshold,
            condition,
            created_at: Utc::now(),
            is_active: true,
        })
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    pub fn activate(&mut self) {
        self.is_active = true;
    }

    pub fn should_trigger(&self, metric_value: f64) -> bool {
        if !self.is_active {
            return false;
        }

        match self.condition {
            AlertCondition::GreaterThan => metric_value > self.threshold,
            AlertCondition::GreaterOrEqual => metric_value >= self.threshold,
            AlertCondition::LessThan => metric_value < self.threshold,
            AlertCondition::LessOrEqual => metric_value <= self.threshold,
        }
    }
}
