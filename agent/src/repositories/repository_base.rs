use crate::traits::repository::Repository;
use crate::{AppError, Result};
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Mutex;
use uuid::Uuid;

/// Repositorio base en memoria que implementa el trait Repository
/// Sigue principio SOLID de Single Responsibility
pub struct RepositoryBase<T, ID> 
where 
    T: Clone + Send + Sync + Debug,
    ID: Clone + Send + Sync + Debug + ToString,
{
    entities: Mutex<HashMap<String, T>>,
    _phantom: std::marker::PhantomData<ID>,
}

impl<T, ID> RepositoryBase<T, ID>
where 
    T: Clone + Send + Sync + Debug,
    ID: Clone + Send + Sync + Debug + ToString,
{
    pub fn new() -> Self {
        Self {
            entities: Mutex::new(HashMap::new()),
            _phantom: std::marker::PhantomData,
        }
    }

    fn get_entities(&self) -> Result<std::sync::MutexGuard<'_, HashMap<String, T>>> {
        self.entities.lock().map_err(|_| AppError::Metrics("Failed to acquire repository lock".into()))
    }
}

impl<T, ID> Default for RepositoryBase<T, ID>
where 
    T: Clone + Send + Sync + Debug,
    ID: Clone + Send + Sync + Debug + ToString,
{
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl<T, ID> Repository<T, ID> for RepositoryBase<T, ID>
where 
    T: Clone + Send + Sync + Debug,
    ID: Clone + Send + Sync + Debug + ToString,
{
    async fn create(&self, entity: T) -> Result<T> {
        let id = Uuid::new_v4().to_string();
        
        // Insertar la entidad con el ID generado
        let mut entities = self.get_entities()?;
        entities.insert(id.clone(), entity.clone());
        
        Ok(entity)
    }

    async fn find_by_id(&self, id: &ID) -> Result<T> {
        let entities = self.get_entities()?;
        let id_str = id.to_string();
        entities.get(&id_str)
            .cloned()
            .ok_or_else(|| AppError::NotFound(format!("Entity with id {} not found", id_str)))
    }

    async fn find_all(&self, limit: Option<usize>) -> Result<Vec<T>> {
        let entities = self.get_entities()?;
        let mut result: Vec<T> = entities.values().cloned().collect();
        
        if let Some(limit) = limit {
            result.truncate(limit);
        }
        
        Ok(result)
    }

    async fn update(&self, id: &ID, entity: T) -> Result<T> {
        let mut entities = self.get_entities()?;
        let id_str = id.to_string();
        
        if !entities.contains_key(&id_str) {
            return Err(AppError::NotFound(format!("Entity with id {} not found", id_str)));
        }

        entities.insert(id_str, entity.clone());
        Ok(entity)
    }

    async fn delete(&self, id: &ID) -> Result<T> {
        let mut entities = self.get_entities()?;
        let id_str = id.to_string();
        
        entities.remove(&id_str)
            .ok_or_else(|| AppError::NotFound(format!("Entity with id {} not found", id_str)))
    }

    async fn count(&self) -> Result<usize> {
        let entities = self.get_entities()?;
        Ok(entities.len())
    }
}
