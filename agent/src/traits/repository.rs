use crate::Result;
use std::fmt::Debug;


#[async_trait::async_trait]
pub trait Repository<T, ID> 
where 
    T: Clone + Send + Sync + Debug,
    ID: Clone + Send + Sync + Debug,
{
    
    async fn create(&self, entity: T) -> Result<T>;
    
    
    async fn find_by_id(&self, id: &ID) -> Result<T>;
    
    
    async fn find_all(&self, limit: Option<usize>) -> Result<Vec<T>>;
    
    
    async fn update(&self, id: &ID, entity: T) -> Result<T>;
    
    
    async fn delete(&self, id: &ID) -> Result<T>;
    
    
    async fn count(&self) -> Result<usize>;
}
