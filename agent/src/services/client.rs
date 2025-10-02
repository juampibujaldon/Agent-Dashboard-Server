use crate::config::settings::Settings;
use crate::models::payloads::MetricPayload;
use crate::{AppError, Result};
use reqwest::Client as ReqwestClient;
use std::time::Duration;

/// Cliente HTTP para comunicar el agente con el backend (Python).
/// - Maneja base_url y api_key
/// - Expone métodos para enviar una métrica, enviar en batch y chequear salud
pub struct Client {
    base_url: String,
    api_key: String,
    http: ReqwestClient,
    // Config básico de reintentos (simple)
    max_retries: u8,
    retry_backoff_ms: u64,
}

impl Client {
    /// Crea un cliente a partir de strings (útil para inyección manual).
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
            max_retries: 2, // 2 reintentos simples (total 3 intentos contando el 1ro)
            retry_backoff_ms: 300, // backoff lineal básico
        }
    }

    /// Crea un cliente usando Settings (conveniente para main/config).
    pub fn from_settings(settings: &Settings) -> Self {
        Self::new(settings.backend_base_url.clone(), settings.api_key.clone())
    }

    /// Envía una métrica (POST /metrics) con JSON.
    /// - Header: x-api-key
    /// - Reintenta simple en errores 5xx o timeouts.
    pub async fn send_metric(&self, metric: &MetricPayload) -> Result<()> {
        if let Err(e) = metric.validate() {
            return Err(AppError::Validation(e));
        }
        let url = self.endpoint("/metrics");
        self.post_with_retries(&url, metric).await
    }

    /// Envía un lote de métricas; por simplicidad, itera una por una.
    /// Devuelve la cantidad exitosa (si falla alguna, corta y retorna error).
    pub async fn send_metrics_batch(&self, metrics: &[MetricPayload]) -> Result<usize> {
        let mut sent = 0usize;
        for m in metrics {
            self.send_metric(m).await?;
            sent += 1;
        }
        Ok(sent)
    }

    /// Chequeo simple de salud (GET /health). Si el backend no lo tiene, podés cambiar a /.
    pub async fn health_check(&self) -> Result<()> {
        let url = self.endpoint("/health");
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

    /// Helper: construye URL completa segura
    fn endpoint(&self, path: &str) -> String {
        let base = self.base_url.trim_end_matches('/');
        let path = path.trim_start_matches('/');
        format!("{base}/{path}")
    }

    /// POST con reintentos básicos ante 5xx/timeouts.
    async fn post_with_retries<T: serde::Serialize>(&self, url: &str, body: &T) -> Result<()> {
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
                    // 4xx: no reintentar (normalmente es bug de request/credenciales)
                    if r.status().is_client_error() {
                        let code = r.status();
                        let text = r.text().await.unwrap_or_default();
                        return Err(AppError::Metrics(format!(
                            "backend responded with {}: {}",
                            code, text
                        )));
                    }
                    // 5xx: reintentar según política
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

                    // Otros estados no comunes: tratarlos como error duro
                    return Err(AppError::Metrics(format!(
                        "unexpected status {}",
                        r.status()
                    )));
                }
                Err(e) => {
                    // Timeout/conexión: reintento hasta max_retries
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
