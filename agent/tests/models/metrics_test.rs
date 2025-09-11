use agent::models::metrics::{Metric, MetricCategory};
use chrono::{Utc};

#[tokio::test]
async fn test_metric_creation() {
    let metric = Metric::new(
        "CPU Usage", 75.5, "%", "server_1", MetricCategory::CPU,
    );
    assert_eq!(metric.name, "CPU Usage");
    assert_eq!(metric.value, 75.5);
    assert_eq!(metric.unit, "%");
    assert_eq!(metric.server_id, "server_1");
    assert!(matches!(metric.category, MetricCategory::CPU));
    assert!(metric.id.is_none());
}

#[tokio::test]
async fn test_metric_with_id() {
    let metric = Metric::new(
        "Memory Usage", 60.0, "%", "server_1", MetricCategory::Memory,
    ).with_id("metric_123");
    assert_eq!(metric.id, Some("metric_123".to_string()));
}

#[tokio::test]
async fn test_metric_critical_detection() {
    let cpu_metric = Metric::new(
        "CPU Usage", 95.5, "%", "server_1", MetricCategory::CPU,
    );
    let memory_metric = Metric::new(
        "Memory Usage", 60.0, "%", "server_1", MetricCategory::Memory,
    );
    assert!(cpu_metric.is_critical());
    assert!(!memory_metric.is_critical());
}

#[test]
fn test_metric_timestamp_is_set() {
    let before = Utc::now();
    let m = Metric::new("CPU", 1.0, "%", "s1", MetricCategory::CPU);
    let after = Utc::now();

    assert!(m.timestamp >= before);
    assert!(m.timestamp <= after);
}

#[test]
fn test_metric_custom_category() {
    let m = Metric::new("Temp", 42.0, "C", "s1", MetricCategory::Custom("Temp".to_string()));
    match &m.category {
        MetricCategory::Custom(name) => assert_eq!(name, "Temp"),
        _ => panic!("Esperaba Custom"),
    }
}
