use crate::Result;
use serde::Serialize;

/// Trait base para clientes HTTP siguiendo principio SOLID de Dependency Inversion
/// Permite diferentes implementaciones de clientes HTTP
#[async_trait::async_trait]
pub trait HttpClient {
    /// Envía datos al endpoint especificado
    async fn post<T: Serialize + Send + Sync>(&self, endpoint: &str, data: &T) -> Result<()>;
    
    /// Verifica la salud del servidor
    async fn health_check(&self) -> Result<()>;
    
    /// Obtiene la URL base del cliente
    fn base_url(&self) -> &str;
}
