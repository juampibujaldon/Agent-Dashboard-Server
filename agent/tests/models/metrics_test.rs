use agent::models::metrics::{Metric, MetricCategory};
use chrono::Utc;

#[tokio::test]
async fn test_metric_creation() {
    let metric = Metric::new(
        "CPU Usage", 75.5, "%", "server_1", MetricCategory::CPU,
    ).unwrap();
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
        "Memory Usage", 85.0, "%", "server_2", MetricCategory::Memory,
    ).unwrap()
    .with_id("metric_123");
    
    assert_eq!(metric.name, "Memory Usage");
    assert_eq!(metric.value, 85.0);
    assert_eq!(metric.unit, "%");
    assert_eq!(metric.server_id, "server_2");
    assert!(matches!(metric.category, MetricCategory::Memory));
    assert_eq!(metric.id, Some("metric_123".to_string()));
}

#[tokio::test]
async fn test_metric_timestamp_is_set() {
    let before = Utc::now();
    let metric = Metric::new(
        "Disk Usage", 90.0, "%", "server_3", MetricCategory::Disk,
    ).unwrap();
    let after = Utc::now();
    
    assert!(metric.timestamp >= before);
    assert!(metric.timestamp <= after);
}

#[tokio::test]
async fn test_metric_critical_detection() {
    let cpu_metric = Metric::new(
        "CPU Usage", 95.0, "%", "server_1", MetricCategory::CPU,
    ).unwrap();
    assert!(cpu_metric.is_critical());
    
    let memory_metric = Metric::new(
        "Memory Usage", 98.0, "%", "server_2", MetricCategory::Memory,
    ).unwrap();
    assert!(memory_metric.is_critical());
    
    let disk_metric = Metric::new(
        "Disk Usage", 95.0, "%", "server_3", MetricCategory::Disk,
    ).unwrap();
    assert!(disk_metric.is_critical());
    
    let normal_metric = Metric::new(
        "CPU Usage", 50.0, "%", "server_4", MetricCategory::CPU,
    ).unwrap();
    assert!(!normal_metric.is_critical());
}

#[tokio::test]
async fn test_metric_custom_category() {
    let custom_metric = Metric::new(
        "Custom Metric", 42.0, "units", "server_5", MetricCategory::Custom("CustomType".to_string()),
    ).unwrap();
    
    assert_eq!(custom_metric.name, "Custom Metric");
    assert_eq!(custom_metric.value, 42.0);
    assert_eq!(custom_metric.unit, "units");
    assert_eq!(custom_metric.server_id, "server_5");
    
    match custom_metric.category {
        MetricCategory::Custom(category_name) => {
            assert_eq!(category_name, "CustomType");
        }
        _ => panic!("Expected Custom category"),
    }
}

#[tokio::test]
async fn test_metric_validation_errors() {
    let result = Metric::new("", 50.0, "%", "server_1", MetricCategory::CPU);
    assert!(result.is_err());
    
    let result = Metric::new("CPU Usage", 50.0, "", "server_1", MetricCategory::CPU);
    assert!(result.is_err());
    
    let result = Metric::new("CPU Usage", 50.0, "%", "", MetricCategory::CPU);
    assert!(result.is_err());
    
    let result = Metric::new("CPU Usage", -10.0, "%", "server_1", MetricCategory::CPU);
    assert!(result.is_err());
    
    
    let result = Metric::new("CPU Usage", f64::NAN, "%", "server_1", MetricCategory::CPU);
    assert!(result.is_err());
    
    
    let result = Metric::new("CPU Usage", f64::INFINITY, "%", "server_1", MetricCategory::CPU);
    assert!(result.is_err());
}