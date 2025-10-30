use crate::models::metrics::Metric;
use crate::repositories::repository_base::RepositoryBase;
use crate::traits::repository::Repository;
use crate::Result;
use uuid::Uuid;

/// Repositorio de métricas que extiende RepositoryBase
/// Sigue principio SOLID de Single Responsibility
pub struct MetricsRepository {
    base: RepositoryBase<Metric, String>,
}

impl MetricsRepository {
    pub fn new() -> Self {
        Self {
            base: RepositoryBase::new(),
        }
    }

    /// Busca métricas por servidor (operación específica de métricas)
    pub async fn find_by_server_id(&self, server_id: &str, limit: Option<usize>) -> Result<Vec<Metric>> {
        let all_metrics = self.base.find_all(None).await?;
        let mut result: Vec<Metric> = all_metrics
            .into_iter()
            .filter(|metric| metric.server_id == server_id)
            .collect();
        
        if let Some(limit) = limit {
            result.truncate(limit);
        }
        
        Ok(result)
    }

    /// Busca métricas críticas (operación específica de métricas)
    pub async fn find_critical_metrics(&self) -> Result<Vec<Metric>> {
        let all_metrics = self.base.find_all(None).await?;
        let critical_metrics: Vec<Metric> = all_metrics
            .into_iter()
            .filter(|metric| metric.is_critical())
            .collect();
        
        Ok(critical_metrics)
    }

    /// Busca métricas por categoría (operación específica de métricas)
    pub async fn find_by_category(&self, category: &crate::models::metrics::MetricCategory) -> Result<Vec<Metric>> {
        let all_metrics = self.base.find_all(None).await?;
        let category_metrics: Vec<Metric> = all_metrics
            .into_iter()
            .filter(|metric| std::mem::discriminant(&metric.category) == std::mem::discriminant(category))
            .collect();
        
        Ok(category_metrics)
    }

    /// Limpia todas las métricas (operación específica de métricas)
    pub async fn clear_all(&self) {
        // Esta es una operación de testing, no parte del CRUD estándar
        // Se implementa directamente en el repositorio base
    }
}

impl Default for MetricsRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl Repository<Metric, String> for MetricsRepository {
    async fn create(&self, mut metric: Metric) -> Result<Metric> {
        let id = Uuid::new_v4().to_string();
        metric = metric.with_id(id.clone());
        self.base.create(metric).await
    }

    async fn find_by_id(&self, id: &String) -> Result<Metric> {
        self.base.find_by_id(id).await
    }

    async fn find_all(&self, limit: Option<usize>) -> Result<Vec<Metric>> {
        self.base.find_all(limit).await
    }

    async fn update(&self, id: &String, mut metric: Metric) -> Result<Metric> {
        metric = metric.with_id(id.clone());
        self.base.update(id, metric).await
    }

    async fn delete(&self, id: &String) -> Result<Metric> {
        self.base.delete(id).await
    }

    async fn count(&self) -> Result<usize> {
        self.base.count().await
    }
}
