use crate::Result;
use std::fmt::Debug;

/// Trait base para repositorios siguiendo principio SOLID de Dependency Inversion
/// Define operaciones CRUD estándar
#[async_trait::async_trait]
pub trait Repository<T, ID> 
where 
    T: Clone + Send + Sync + Debug,
    ID: Clone + Send + Sync + Debug,
{
    /// Crea una nueva entidad
    async fn create(&self, entity: T) -> Result<T>;
    
    /// Busca una entidad por ID
    async fn find_by_id(&self, id: &ID) -> Result<T>;
    
    /// Obtiene todas las entidades
    async fn find_all(&self, limit: Option<usize>) -> Result<Vec<T>>;
    
    /// Actualiza una entidad existente
    async fn update(&self, id: &ID, entity: T) -> Result<T>;
    
    /// Elimina una entidad
    async fn delete(&self, id: &ID) -> Result<T>;
    
    /// Cuenta el total de entidades
    async fn count(&self) -> Result<usize>;
}
