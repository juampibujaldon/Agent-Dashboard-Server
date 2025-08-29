use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Error de metricas: {0}")]
    Metrics(String),

    #[error("Error de validacion: {0}")]
    Validation(String),
}