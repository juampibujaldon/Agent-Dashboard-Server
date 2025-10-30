use crate::traits::http_client::HttpClient;
use crate::{AppError, Result};
use reqwest::Client as ReqwestClient;
use serde::Serialize;
use std::time::Duration;

/// Cliente HTTP base que implementa el trait HttpClient
/// Elimina duplicación de código siguiendo principio DRY
pub struct HttpClientBase {
    base_url: String,
    api_key: String,
    http: ReqwestClient,
    max_retries: u8,
    retry_backoff_ms: u64,
}

impl HttpClientBase {
    /// Crea un nuevo cliente HTTP base
    pub fn new(base_url: impl Into<String>, api_key: impl Into<String>) -> Result<Self> {
        let http = ReqwestClient::builder()
            .user_agent("agent-rust/0.1.0")
            .timeout(Duration::from_secs(10))
            .build()
            .map_err(|e| AppError::Metrics(format!("Failed to build HTTP client: {}", e)))?;

        Ok(Self {
            base_url: base_url.into(),
            api_key: api_key.into(),
            http,
            max_retries: 2,
            retry_backoff_ms: 300,
        })
    }

    /// Construye la URL completa del endpoint
    fn build_url(&self, endpoint: &str) -> String {
        let base = self.base_url.trim_end_matches('/');
        let endpoint = endpoint.trim_start_matches('/');
        format!("{base}/{endpoint}")
    }

    /// Envía POST con reintentos automáticos
    async fn post_with_retries<T: Serialize + Send + Sync>(&self, url: &str, body: &T) -> Result<()> {
        let mut attempt: u8 = 0;

        loop {
            let resp = self
                .http
                .post(url)
                .header("x-api-key", &self.api_key)
                .json(body)
                .send()
                .await;

            match resp {
                Ok(r) if r.status().is_success() => return Ok(()),
                Ok(r) => {
                    if r.status().is_client_error() {
                        let code = r.status();
                        let text = r.text().await.unwrap_or_default();
                        return Err(AppError::Metrics(format!(
                            "backend responded with {}: {}",
                            code, text
                        )));
                    }
                    if r.status().is_server_error() {
                        if attempt >= self.max_retries {
                            return Err(AppError::Metrics(format!(
                                "backend responded with {} after {} retries",
                                r.status(),
                                attempt
                            )));
                        }
                        attempt += 1;
                        tokio::time::sleep(Duration::from_millis(self.retry_backoff_ms)).await;
                        continue;
                    }

                    return Err(AppError::Metrics(format!(
                        "unexpected status {}",
                        r.status()
                    )));
                }
                Err(e) => {
                    if attempt >= self.max_retries {
                        return Err(AppError::Metrics(format!("http error after retries: {e}")));
                    }
                    attempt += 1;
                    tokio::time::sleep(Duration::from_millis(self.retry_backoff_ms)).await;
                    continue;
                }
            }
        }
    }
}

#[async_trait::async_trait]
impl HttpClient for HttpClientBase {
    async fn post<T: Serialize + Send + Sync>(&self, endpoint: &str, data: &T) -> Result<()> {
        let url = self.build_url(endpoint);
        self.post_with_retries(&url, data).await
    }

    async fn health_check(&self) -> Result<()> {
        let url = self.build_url("/health");
        let res = self
            .http
            .get(&url)
            .header("x-api-key", &self.api_key)
            .send()
            .await
            .map_err(|e| AppError::Metrics(format!("http error: {e}")))?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(AppError::Metrics(format!(
                "health_check status {}",
                res.status()
            )))
        }
    }

    fn base_url(&self) -> &str {
        &self.base_url
    }
}

