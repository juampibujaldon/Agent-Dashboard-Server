use crate::models::server::Server;
use crate::repositories::repository_base::RepositoryBase;
use crate::traits::repository::Repository;
use crate::Result;
use uuid::Uuid;

pub struct ServersRepository {
    base: RepositoryBase<Server, String>,
}

impl ServersRepository {
    pub fn new() -> Self {
        Self {
            base: RepositoryBase::new(),
        }
    }

    
    pub async fn find_active_servers(&self) -> Result<Vec<Server>> {
        let all_servers = self.base.find_all(None).await?;
        let active_servers: Vec<Server> = all_servers
            .into_iter()
            .filter(|server| server.is_active)
            .collect();
        
        Ok(active_servers)
    }

    
    pub async fn find_online_servers(&self) -> Result<Vec<Server>> {
        let all_servers = self.base.find_all(None).await?;
        let online_servers: Vec<Server> = all_servers
            .into_iter()
            .filter(|server| server.is_online())
            .collect();
        
        Ok(online_servers)
    }

    
    pub async fn find_by_hostname(&self, hostname: &str) -> Result<Option<Server>> {
        let all_servers = self.base.find_all(None).await?;
        let server = all_servers
            .into_iter()
            .find(|server| server.hostname == hostname);
        
        Ok(server)
    }

    
    pub async fn find_by_ip(&self, ip_address: &str) -> Result<Option<Server>> {
        let all_servers = self.base.find_all(None).await?;
        let server = all_servers
            .into_iter()
            .find(|server| server.ip_address == ip_address);
        
        Ok(server)
    }
}

impl Default for ServersRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl Repository<Server, String> for ServersRepository {
    async fn create(&self, mut server: Server) -> Result<Server> {
        let id = Uuid::new_v4().to_string();
        server = server.with_id(id.clone());
        self.base.create(server).await
    }

    async fn find_by_id(&self, id: &String) -> Result<Server> {
        self.base.find_by_id(id).await
    }

    async fn find_all(&self, limit: Option<usize>) -> Result<Vec<Server>> {
        self.base.find_all(limit).await
    }

    async fn update(&self, id: &String, mut server: Server) -> Result<Server> {
        server = server.with_id(id.clone());
        self.base.update(id, server).await
    }

    async fn delete(&self, id: &String) -> Result<Server> {
        self.base.delete(id).await
    }

    async fn count(&self) -> Result<usize> {
        self.base.count().await
    }
}