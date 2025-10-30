use crate::models::system_metrics::SystemMetrics;
use crate::traits::monitor::SystemMonitor;
use rand::Rng;
use chrono::Timelike;


pub struct FakeMonitor {
    servers: Vec<String>,
    base_metrics: std::collections::HashMap<String, SystemMetrics>,
}

impl FakeMonitor {
    
    pub fn new() -> Self {
        let servers = vec![
            "server-web-01".to_string(),
            "server-db-01".to_string(),
            "server-api-01".to_string(),
        ];
        
        Self::with_servers(servers)
    }

        pub fn with_servers(servers: Vec<String>) -> Self {
        let mut base_metrics = std::collections::HashMap::new();
        
        
        for server_id in &servers {
            let base = Self::generate_base_metrics(server_id);
            base_metrics.insert(server_id.clone(), base);
        }
        
        Self {
            servers,
            base_metrics,
        }
    }

    
    pub fn get_servers(&self) -> Vec<String> {
        self.servers.clone()
    }

    
    fn generate_base_metrics(server_id: &str) -> SystemMetrics {
        let mut rng = rand::thread_rng();
        
        
        let (cpu_base, ram_base, disk_base, temp_base) = if server_id.contains("web") {
            (45.0, 60.0, 40.0, 50.0) 
        } else if server_id.contains("db") {
            (35.0, 80.0, 70.0, 55.0) 
        } else if server_id.contains("api") {
            (55.0, 45.0, 30.0, 48.0) 
        } else {
            (50.0, 50.0, 50.0, 50.0) //
        };
        
        SystemMetrics::new(
            cpu_base + rng.gen_range(-10.0..10.0),
            ram_base + rng.gen_range(-15.0..15.0),
            disk_base + rng.gen_range(-20.0..20.0),
            temp_base + rng.gen_range(-10.0..10.0),
        )
    }

    
    fn generate_realistic_fluctuation(&self, base: f32, hour: f32) -> f32 {
        let mut rng = rand::thread_rng();
        
        
        let random_fluctuation = rng.gen_range(-5.0..5.0);
        
        
        let time_factor = match hour {
            h if h >= 9.0 && h <= 17.0 => 0.3, 
            h if h >= 18.0 && h <= 22.0 => 0.1, 
            h if h >= 23.0 || h <= 6.0 => -0.4, 
            _ => 0.2, 
        };
        
        let final_value = base + random_fluctuation + time_factor;
        
        
        final_value.max(0.0).min(100.0)
    }

    
    pub fn generate_server_metrics(&mut self, server_id: &str) -> SystemMetrics {
        let base = self.base_metrics.get(server_id).unwrap().clone();
        
        
        let _server_type = if server_id.contains("web") {
            "web"
        } else if server_id.contains("db") {
            "db"
        } else if server_id.contains("api") {
            "api"
        } else {
            "generic"
        };
        
        
        let now = chrono::Utc::now();
        let hour = now.hour() as f32 + now.minute() as f32 / 60.0;
        
        
        let cpu_usage = self.generate_realistic_fluctuation(base.cpu_usage, hour);
        let ram_usage = self.generate_realistic_fluctuation(base.ram_usage, hour);
        let disk_space = self.generate_realistic_fluctuation(base.disk_space, hour);
        let temperature = self.generate_realistic_fluctuation(base.temperature, hour);
        
        SystemMetrics::new(cpu_usage, ram_usage, disk_space, temperature)
    }

    
    pub fn collect_all_metrics(&mut self) -> std::collections::HashMap<String, SystemMetrics> {
        let mut all_metrics = std::collections::HashMap::new();
        
        for server_id in self.servers.clone() {
            let metrics = self.generate_server_metrics(&server_id);
            all_metrics.insert(server_id, metrics);
        }
        
        all_metrics
    }
}

impl SystemMonitor for FakeMonitor {
    
    fn collect(&mut self) -> SystemMetrics {
        if let Some(first_server) = self.servers.first().cloned() {
            self.generate_server_metrics(&first_server)
        } else {
            SystemMetrics::new(0.0, 0.0, 0.0, 0.0)
        }
    }

    fn get_servers(&self) -> Vec<String> {
        self.servers.clone()
    }
}

impl Default for FakeMonitor {
    fn default() -> Self {
        Self::new()
    }
}
