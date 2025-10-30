use thiserror::Error;


#[derive(Error, Debug)]
pub enum AppError {
    #[error("Error de métricas: {0}")]
    Metrics(String),

    #[error("Error de validación: {0}")]
    Validation(String),

    #[error("Recurso no encontrado: {0}")]
    NotFound(String),

    #[error("Error de solicitud HTTP: {0}")]
    RequestError(String),
}
