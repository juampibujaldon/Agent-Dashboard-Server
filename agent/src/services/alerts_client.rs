use crate::{Result, AppError};
use crate::models::alert::Alert;
use reqwest::Client as ReqwestClient;
use std::time::Duration;

pub struct AlertsClient {
    base_url: String,
    api_key: String,
    http: ReqwestClient,
    max_retries: u8,
    retry_backoff_ms: u64,
}

impl AlertsClient {
    pub fn new(base_url: impl Into<String>, api_key: impl Into<String>) -> Self {
        let http = ReqwestClient::builder()
            .user_agent("agent-rust/0.1.0")
            .timeout(Duration::from_secs(10))
            .build()
            .expect("failed to build reqwest client");

        Self {
            base_url: base_url.into(),
            api_key: api_key.into(),
            http,
            max_retries: 2,
            retry_backoff_ms: 300,
        }
    }

    pub async fn send_alert(&self, alert: &Alert) -> Result<()> {
        // Validaciones mínimas
        if alert.server_id.trim().is_empty() {
            return Err(AppError::Validation("alert.server_id vacío".into()));
        }
        if alert.metric_type.trim().is_empty() {
            return Err(AppError::Validation("alert.metric_type vacío".into()));
        }

        let url = self.endpoint("/alerts");
        self.post_with_retries(&url, alert).await
    }

    pub async fn send_alerts_batch(&self, alerts: &[Alert]) -> Result<usize> {
        let mut sent = 0usize;
        for a in alerts {
            self.send_alert(a).await?;
            sent += 1;
        }
        Ok(sent)
    }

    fn endpoint(&self, path: &str) -> String {
        let base = self.base_url.trim_end_matches('/');
        let path = path.trim_start_matches('/');
        format!("{base}/{path}")
    }

    async fn post_with_retries<T: serde::Serialize>(&self, url: &str, body: &T) -> Result<()> {
        let mut attempt: u8 = 0;
        loop {
            let resp = self.http
                .post(url)
                .header("x-api-key", &self.api_key)
                .json(body)
                .send()
                .await;

            match resp {
                Ok(r) if r.status().is_success() => return Ok(()),
                Ok(r) if r.status().is_client_error() => {
                    let code = r.status();
                    let text = r.text().await.unwrap_or_default();
                    return Err(AppError::Metrics(format!("backend responded with {}: {}", code, text)));
                }
                Ok(r) if r.status().is_server_error() => {
                    if attempt >= self.max_retries {
                        return Err(AppError::Metrics(format!(
                            "backend responded with {} after {} retries", r.status(), attempt
                        )));
                    }
                    attempt += 1;
                    tokio::time::sleep(Duration::from_millis(self.retry_backoff_ms)).await;
                }
                Ok(r) => {
                    return Err(AppError::Metrics(format!("unexpected status {}", r.status())));
                }
                Err(e) => {
                    if attempt >= self.max_retries {
                        return Err(AppError::Metrics(format!("http error after retries: {e}")));
                    }
                    attempt += 1;
                    tokio::time::sleep(Duration::from_millis(self.retry_backoff_ms)).await;
                }
            }
        }
    }
}
