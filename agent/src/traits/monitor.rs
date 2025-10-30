use crate::models::system_metrics::SystemMetrics;

/// Trait para monitores del sistema siguiendo principio SOLID de Dependency Inversion
/// Permite diferentes implementaciones de monitores (real, fake, etc.)
pub trait SystemMonitor {
    /// Recolecta métricas del sistema
    fn collect(&mut self) -> SystemMetrics;
    
    /// Obtiene la lista de servidores monitoreados
    fn get_servers(&self) -> Vec<String>;
}

