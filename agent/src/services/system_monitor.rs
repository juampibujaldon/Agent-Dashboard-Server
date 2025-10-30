use crate::models::system_metrics::SystemMetrics;
use crate::traits::monitor::SystemMonitor;
use sysinfo::{Components, Disks, System};

pub struct SysinfoMonitor {
    system: System,
    disks: Disks,
    components: Components,
}

impl SysinfoMonitor {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_cpu();
        system.refresh_memory();

        let mut disks = Disks::new_with_refreshed_list();
        disks.refresh();

        let mut components = Components::new_with_refreshed_list();
        components.refresh();

        Self {
            system,
            disks,
            components,
        }
    }

    fn clamp_percent(value: f32) -> f32 {
        value.clamp(0.0, 100.0)
    }
}

impl SystemMonitor for SysinfoMonitor {
    fn collect(&mut self) -> SystemMetrics {
        self.system.refresh_cpu();
        self.system.refresh_memory();
        self.disks.refresh();
        self.components.refresh();

        let cpu_usage = Self::clamp_percent(self.system.global_cpu_info().cpu_usage());

        let total_memory = self.system.total_memory() as f32;
        let used_memory = self.system.used_memory() as f32;
        let ram_usage = if total_memory > 0.0 {
            Self::clamp_percent((used_memory / total_memory) * 100.0)
        } else {
            0.0
        };

        let mut total_disk = 0.0f32;
        let mut used_disk = 0.0f32;
        for disk in self.disks.list() {
            let total = disk.total_space() as f32;
            let available = disk.available_space() as f32;
            if total > 0.0 {
                total_disk += total;
                used_disk += total - available;
            }
        }
        let disk_space = if total_disk > 0.0 {
            Self::clamp_percent((used_disk / total_disk) * 100.0)
        } else {
            0.0
        };

        let temperature = self
            .components
            .list()
            .iter()
            .map(|component| component.temperature())
            .fold(0.0f32, f32::max);

        SystemMetrics::new(cpu_usage, ram_usage, disk_space, temperature)
    }

    fn get_servers(&self) -> Vec<String> {
        vec!["localhost".to_string()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_percent_bounds() {
        assert!((SysinfoMonitor::clamp_percent(-10.0) - 0.0).abs() < f32::EPSILON);
        assert!((SysinfoMonitor::clamp_percent(200.0) - 100.0).abs() < f32::EPSILON);
        assert!((SysinfoMonitor::clamp_percent(55.5) - 55.5).abs() < f32::EPSILON);
    }
}
