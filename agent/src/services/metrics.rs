
use crate::models::metrics::Metrica;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn recolectar_metricas() -> Metrica {
    // TODO: Reemplazar esto con una librería como `sysinfo` para obtener datos reales.
    let timestamp_actual = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    Metrica {
        servidor_id: "servidor-ejemplo-01".to_string(), 
        timestamp: timestamp_actual,
        cpu_usage: 15.5, 
        ram_usage: 2048.0, 
    }
}

