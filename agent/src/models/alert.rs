use serde::{Deserialize, Serialize};

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
    pub metric_type: String, // "cpu_usage", "ram_usage", etc.
    pub threshold: f64,
    pub condition: AlertCondition,
}

impl Alert {
    pub fn new(
        server_id: impl Into<String>,
        metric_type: impl Into<String>,
        threshold: f64,
        condition: AlertCondition,
    ) -> Self {
        Self {
            id: None,
            server_id: server_id.into(),
            metric_type: metric_type.into(),
            threshold,
            condition,
        }
    }
}
