use crate::{models::metrics::Metric, Result};
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Default)]
pub struct MetricsRepository {
    metrics: Mutex<HashMap<String, Metric>>,
}

impl MetricsRepository {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn create(&self, mut metric: Metric) -> Result<Metric> {
        let id = Uuid::new_v4().to_string();
        metric = metric.with_id(id.clone());

        let mut metrics = self.metrics.lock()
            .map_err(|_| crate::AppError::Metrics("Failed to acquire repository lock".into()))?;
        metrics.insert(id, metric.clone());

        Ok(metric)
    }

    pub async fn find_by_id(&self, id: &str) -> Result<Metric> {
        let metrics = self.metrics.lock()
            .map_err(|_| crate::AppError::Metrics("Failed to acquire repository lock".into()))?;
        metrics.get(id)
            .cloned()
            .ok_or_else(|| crate::AppError::NotFound(format!("Metric with id {} not found", id)))
    }

    pub async fn find_all(&self, limit: Option<usize>) -> Result<Vec<Metric>> {
        let metrics = self.metrics.lock()
            .map_err(|_| crate::AppError::Metrics("Failed to acquire repository lock".into()))?;
        
        let mut result: Vec<Metric> = metrics.values().cloned().collect();
        
        if let Some(limit) = limit {
            result.truncate(limit);
        }
        
        Ok(result)
    }

    pub async fn update(&self, id: &str, mut metric: Metric) -> Result<Metric> {
        let mut metrics = self.metrics.lock()
            .map_err(|_| crate::AppError::Metrics("Failed to acquire repository lock".into()))?;
        
        if !metrics.contains_key(id) {
            return Err(crate::AppError::NotFound(format!("Metric with id {} not found", id)));
        }

        metric = metric.with_id(id.to_string());
        metrics.insert(id.to_string(), metric.clone());
        
        Ok(metric)
    }

    pub async fn delete(&self, id: &str) -> Result<Metric> {
        let mut metrics = self.metrics.lock()
            .map_err(|_| crate::AppError::Metrics("Failed to acquire repository lock".into()))?;
        
        metrics.remove(id)
            .ok_or_else(|| crate::AppError::NotFound(format!("Metric with id {} not found", id)))
    }

    pub async fn find_by_server_id(&self, server_id: &str, limit: Option<usize>) -> Result<Vec<Metric>> {
        let metrics = self.metrics.lock()
            .map_err(|_| crate::AppError::Metrics("Failed to acquire repository lock".into()))?;
        
        let mut result: Vec<Metric> = metrics.values()
            .filter(|metric| metric.server_id == server_id)
            .cloned()
            .collect();
        
        // Aplicar límite si se especifica
        if let Some(limit) = limit {
            result.truncate(limit);
        }
        
        Ok(result)
    }
}
