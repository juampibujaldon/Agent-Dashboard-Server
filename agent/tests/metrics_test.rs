use agent::models::metrics::{Metric, MetricCategory};

#[tokio::test]
async fn test_metric_creation() {
    let metric = Metric::new(
        "CPU Usage",
        75.5,
        "%",
        "server_1",
        MetricCategory::CPU,
    );
    
    assert_eq!(metric.name, "CPU Usage");
    assert_eq!(metric.value, 75.5);
    assert_eq!(metric.unit, "%");
    assert_eq!(metric.server_id, "server_1");
    assert!(matches!(metric.category, MetricCategory::CPU));
    assert!(metric.id.is_none()); // No debe tener ID al crear
}

#[tokio::test]
async fn test_metric_with_id() {
    let metric = Metric::new(
        "Memory Usage",
        60.0,
        "%",
        "server_1",
        MetricCategory::Memory,
    ).with_id("metric_123");
    
    assert_eq!(metric.id, Some("metric_123".to_string()));
}

#[tokio::test]
async fn test_metric_critical_detection() {
    let cpu_metric = Metric::new(
        "CPU Usage",
        95.5, // Crítico
        "%",
        "server_1",
        MetricCategory::CPU,
    );
    
    let memory_metric = Metric::new(
        "Memory Usage",
        60.0, // No crítico
        "%",
        "server_1",
        MetricCategory::Memory,
    );
    
    assert!(cpu_metric.is_critical()); // CPU > 90% es crítico
    assert!(!memory_metric.is_critical()); // Memory > 95% es crítico
}