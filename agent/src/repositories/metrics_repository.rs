use crate::{models::metrics::Metric, Result};
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Default)]
pub struct MetricsRepository{
    metrics: Mutex<HashMap<String, Metric>>,

}

impl MetricsRepository {

    pub fn new() -> Self{
        Self::default()
    }
    
    pub async fn create(&self, mut metric: Metric) -> Result<Metric>{
        let id = Uuid::new_v4().to_string();
        metric = metric.with_id(id.clone());

        let mut metrics = self.metrics.lock().unwrap();
        metrics.insert(id, metric.clone());

        Ok(metric)
    }
}
