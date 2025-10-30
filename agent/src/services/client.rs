use crate::config::settings::Settings;
use crate::models::payloads::MetricPayload;
use crate::services::http_client_base::HttpClientBase;
use crate::traits::http_client::HttpClient;
use crate::{AppError, Result};

pub struct Client {
    http_client: HttpClientBase,
}

impl Client {
    pub fn new(base_url: impl Into<String>, api_key: impl Into<String>) -> Result<Self> {
        let http_client = HttpClientBase::new(base_url, api_key)?;
        Ok(Self { http_client })
    }

    
    pub fn from_settings(settings: &Settings) -> Self {
        Self::new(settings.backend_base_url.clone(), settings.api_key.clone()).unwrap()
    }

    pub async fn send_metric(&self, metric: &MetricPayload) -> Result<()> {
        if let Err(e) = metric.validate() {
            return Err(AppError::Validation(e));
        }
        self.http_client.post("/metrics", metric).await
    }


    pub async fn send_metrics_batch(&self, metrics: &[MetricPayload]) -> Result<usize> {
        let mut sent = 0usize;
        for m in metrics {
            self.send_metric(m).await?;
            sent += 1;
        }
        Ok(sent)
    }

    pub async fn health_check(&self) -> Result<()> {
        self.http_client.health_check().await
    }
}