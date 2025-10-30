use crate::models::system_metrics::SystemMetrics;

pub trait SystemMonitor {
    fn collect(&mut self) -> SystemMetrics;
    
    fn get_servers(&self) -> Vec<String>;
}

