use crate::models::alert::Alert;
use crate::services::http_client_base::HttpClientBase;
use crate::traits::http_client::HttpClient;
use crate::{AppError, Result};

/// Cliente especializado para alertas que usa HttpClientBase
/// Sigue principio SOLID de Single Responsibility
pub struct AlertsClient {
    http_client: HttpClientBase,
}

impl AlertsClient {
    pub fn new(base_url: impl Into<String>, api_key: impl Into<String>) -> Result<Self> {
        let http_client = HttpClientBase::new(base_url, api_key)?;
        Ok(Self { http_client })
    }

    pub async fn send_alert(&self, alert: &Alert) -> Result<()> {
        if alert.server_id.trim().is_empty() {
            return Err(AppError::Validation("alert.server_id vacío".into()));
        }
        if alert.metric_type.trim().is_empty() {
            return Err(AppError::Validation("alert.metric_type vacío".into()));
        }

        self.http_client.post("/alerts", alert).await
    }

    pub async fn send_alerts_batch(&self, alerts: &[Alert]) -> Result<usize> {
        let mut sent = 0usize;
        for a in alerts {
            self.send_alert(a).await?;
            sent += 1;
        }
        Ok(sent)
    }
}
