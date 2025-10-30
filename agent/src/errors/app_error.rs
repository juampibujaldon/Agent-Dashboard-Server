use thiserror::Error;

/// Error consolidado siguiendo principio KISS
/// Un solo tipo de error para simplificar el manejo
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Error de métricas: {0}")]
    Metrics(String),

    #[error("Error de validación: {0}")]
    Validation(String),

    #[error("Recurso no encontrado: {0}")]
    NotFound(String),
}
