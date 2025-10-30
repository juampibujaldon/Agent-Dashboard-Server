use crate::Result;
use serde::Serialize;


#[async_trait::async_trait]
pub trait HttpClient {
    async fn post<T: Serialize + Send + Sync>(&self, endpoint: &str, data: &T) -> Result<()>;
    
    async fn health_check(&self) -> Result<()>;
    
    fn base_url(&self) -> &str;
}
