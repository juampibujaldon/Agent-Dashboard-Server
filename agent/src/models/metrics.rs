
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Metrica {
    pub servidor_id: String,
    pub timestamp: u64,
    pub cpu_usage: f32,
    pub ram_usage: f32,
}

