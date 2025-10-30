use crate::models::alert::Alert;
use crate::repositories::repository_base::RepositoryBase;
use crate::traits::repository::Repository;
use crate::Result;
use uuid::Uuid;

pub struct AlertsRepository {
    base: RepositoryBase<Alert, String>,
}

impl AlertsRepository {
    pub fn new() -> Self {
        Self {
            base: RepositoryBase::new(),
        }
    }

    pub async fn find_by_server_id(&self, server_id: &str, limit: Option<usize>) -> Result<Vec<Alert>> {
        let all_alerts = self.base.find_all(None).await?;
        let mut result: Vec<Alert> = all_alerts
            .into_iter()
            .filter(|alert| alert.server_id == server_id)
            .collect();
        
        if let Some(limit) = limit {
            result.truncate(limit);
        }
        
        Ok(result)
    }

    pub async fn find_active_alerts(&self) -> Result<Vec<Alert>> {
        let all_alerts = self.base.find_all(None).await?;
        let active_alerts: Vec<Alert> = all_alerts
            .into_iter()
            .filter(|alert| alert.is_active)
            .collect();
        
        Ok(active_alerts)
    }

    pub async fn find_by_metric_type(&self, metric_type: &str) -> Result<Vec<Alert>> {
        let all_alerts = self.base.find_all(None).await?;
        let metric_alerts: Vec<Alert> = all_alerts
            .into_iter()
            .filter(|alert| alert.metric_type == metric_type)
            .collect();
        
        Ok(metric_alerts)
    }
}

impl Default for AlertsRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl Repository<Alert, String> for AlertsRepository {
    async fn create(&self, mut alert: Alert) -> Result<Alert> {
        let id = Uuid::new_v4().to_string();
        alert = alert.with_id(id.clone());
        self.base.create(alert).await
    }

    async fn find_by_id(&self, id: &String) -> Result<Alert> {
        self.base.find_by_id(id).await
    }

    async fn find_all(&self, limit: Option<usize>) -> Result<Vec<Alert>> {
        self.base.find_all(limit).await
    }

    async fn update(&self, id: &String, mut alert: Alert) -> Result<Alert> {
        alert = alert.with_id(id.clone());
        self.base.update(id, alert).await
    }

    async fn delete(&self, id: &String) -> Result<Alert> {
        self.base.delete(id).await
    }

    async fn count(&self) -> Result<usize> {
        self.base.count().await
    }
}
