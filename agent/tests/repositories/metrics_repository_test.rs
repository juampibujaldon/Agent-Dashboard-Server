use agent::models::metrics::{Metric, MetricCategory};
use agent::repositories::metrics_repository::MetricsRepository;
use agent::traits::repository::Repository;

#[tokio::test]
async fn test_repository_creation() {
    let repository = MetricsRepository::new();

    let metric = Metric::new(
        "CPU Usage",
        75.5,
        "%",
        "server_1",
        MetricCategory::CPU,
    ).unwrap();

    let saved_metric = repository.create(metric).await.unwrap();

    assert!(saved_metric.id.is_some());
    assert_eq!(saved_metric.name, "CPU Usage");
}

#[tokio::test]
async fn test_metrics_repository_find_by_id() {
    let repository = MetricsRepository::new();

    let metric = Metric::new(
        "Memory Usage",
        85.0,
        "%",
        "server_2",
        MetricCategory::Memory,
    ).unwrap();

    let saved_metric = repository.create(metric).await.unwrap();
    let metric_id = saved_metric.id.clone().unwrap();
    let found_metric = repository.find_by_id(&metric_id).await.unwrap();

    assert_eq!(found_metric.id, saved_metric.id);
    assert_eq!(found_metric.name, "Memory Usage");
    assert_eq!(found_metric.value, 85.0);
    assert_eq!(found_metric.unit, "%");
    assert_eq!(found_metric.server_id, "server_2");
    assert!(matches!(found_metric.category, MetricCategory::Memory));
}

#[tokio::test]
async fn test_metrics_repository_find_all() {
    let repository = MetricsRepository::new();

    // Clear any existing metrics
    repository.clear_all().await;

    let metric1 = Metric::new(
        "CPU Usage",
        75.5,
        "%",
        "server_1",
        MetricCategory::CPU,
    ).unwrap();

    let metric2 = Metric::new(
        "Memory Usage",
        85.0,
        "%",
        "server_2",
        MetricCategory::Memory,
    ).unwrap();

    repository.create(metric1).await.unwrap();
    repository.create(metric2).await.unwrap();

    let all_metrics = repository.find_all(None).await.unwrap();

    assert_eq!(all_metrics.len(), 2);
    
    // Check that both metrics are present
    let metric_names: Vec<String> = all_metrics.iter()
        .map(|m| m.name.clone())
        .collect();
    
    assert!(metric_names.contains(&"CPU Usage".to_string()));
    assert!(metric_names.contains(&"Memory Usage".to_string()));
}

#[tokio::test]
async fn test_metrics_repository_find_by_server_id() {
    let repository = MetricsRepository::new();

    let metric1 = Metric::new(
        "CPU Usage",
        75.5,
        "%",
        "server_1",
        MetricCategory::CPU,
    ).unwrap();

    let metric2 = Metric::new(
        "Memory Usage",
        85.0,
        "%",
        "server_1",
        MetricCategory::Memory,
    ).unwrap();

    let metric3 = Metric::new(
        "Disk Usage",
        90.0,
        "%",
        "server_2",
        MetricCategory::Disk,
    ).unwrap();

    repository.create(metric1).await.unwrap();
    repository.create(metric2).await.unwrap();
    repository.create(metric3).await.unwrap();

    let server_1_metrics = repository.find_by_server_id("server_1", None).await.unwrap();
    let server_2_metrics = repository.find_by_server_id("server_2", None).await.unwrap();

    assert_eq!(server_1_metrics.len(), 2);
    assert_eq!(server_2_metrics.len(), 1);

    let server_1_names: Vec<String> = server_1_metrics.iter()
        .map(|m| m.name.clone())
        .collect();
    assert!(server_1_names.contains(&"CPU Usage".to_string()));
    assert!(server_1_names.contains(&"Memory Usage".to_string()));

    assert_eq!(server_2_metrics[0].name, "Disk Usage");
}

#[tokio::test]
async fn test_metrics_repository_find_by_category() {
    let repository = MetricsRepository::new();

    let cpu_metric1 = Metric::new(
        "CPU Usage 1",
        75.5,
        "%",
        "server_1",
        MetricCategory::CPU,
    ).unwrap();

    let cpu_metric2 = Metric::new(
        "CPU Usage 2",
        80.0,
        "%",
        "server_2",
        MetricCategory::CPU,
    ).unwrap();

    let memory_metric = Metric::new(
        "Memory Usage",
        85.0,
        "%",
        "server_1",
        MetricCategory::Memory,
    ).unwrap();

    repository.create(cpu_metric1).await.unwrap();
    repository.create(cpu_metric2).await.unwrap();
    repository.create(memory_metric).await.unwrap();

    let cpu_metrics = repository.find_by_category(&MetricCategory::CPU).await.unwrap();
    let memory_metrics = repository.find_by_category(&MetricCategory::Memory).await.unwrap();

    assert_eq!(cpu_metrics.len(), 2);
    assert_eq!(memory_metrics.len(), 1);

    let cpu_names: Vec<String> = cpu_metrics.iter()
        .map(|m| m.name.clone())
        .collect();
    assert!(cpu_names.contains(&"CPU Usage 1".to_string()));
    assert!(cpu_names.contains(&"CPU Usage 2".to_string()));

    assert_eq!(memory_metrics[0].name, "Memory Usage");
}

#[tokio::test]
async fn test_metrics_repository_update() {
    let repository = MetricsRepository::new();

    let metric = Metric::new(
        "CPU Usage",
        75.5,
        "%",
        "server_1",
        MetricCategory::CPU,
    ).unwrap();

    let saved_metric = repository.create(metric).await.unwrap();
    let mut updated_metric = saved_metric.clone();
    updated_metric.value = 85.0;
    updated_metric.name = "Updated CPU Usage".to_string();

    let metric_id = saved_metric.id.clone().unwrap();
    let result = repository.update(&metric_id, updated_metric).await.unwrap();

    assert_eq!(result.id, saved_metric.id);
    assert_eq!(result.name, "Updated CPU Usage");
    assert_eq!(result.value, 85.0);
    assert_eq!(result.unit, "%");
    assert_eq!(result.server_id, "server_1");
}

#[tokio::test]
async fn test_metrics_repository_delete() {
    let repository = MetricsRepository::new();

    let metric = Metric::new(
        "CPU Usage",
        75.5,
        "%",
        "server_1",
        MetricCategory::CPU,
    ).unwrap();

    let saved_metric = repository.create(metric).await.unwrap();
    let metric_id = saved_metric.id.unwrap().clone();

    let found_metric = repository.find_by_id(&metric_id).await.unwrap();
    assert_eq!(found_metric.id, Some(metric_id.clone()));

    let deleted = repository.delete(&metric_id).await.unwrap();
    assert!(deleted.id.is_some());

    let result = repository.find_by_id(&metric_id).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_metrics_repository_delete_nonexistent() {
    let repository = MetricsRepository::new();

    let result = repository.delete(&"nonexistent_id".to_string()).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_metrics_repository_find_by_id_nonexistent() {
    let repository = MetricsRepository::new();

    let result = repository.find_by_id(&"nonexistent_id".to_string()).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_metrics_repository_update_nonexistent() {
    let repository = MetricsRepository::new();

    let metric = Metric::new(
        "CPU Usage",
        75.5,
        "%",
        "server_1",
        MetricCategory::CPU,
    ).unwrap();

    let result = repository.update(&"nonexistent_id".to_string(), metric).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_metrics_repository_clear_all() {
    let repository = MetricsRepository::new();

    
    let metric1 = Metric::new(
        "CPU Usage",
        75.5,
        "%",
        "server_1",
        MetricCategory::CPU,
    ).unwrap();

    let metric2 = Metric::new(
        "Memory Usage",
        85.0,
        "%",
        "server_2",
        MetricCategory::Memory,
    ).unwrap();

    repository.create(metric1).await.unwrap();
    repository.create(metric2).await.unwrap();

    let all_metrics = repository.find_all(None).await.unwrap();
    assert_eq!(all_metrics.len(), 2);

    repository.clear_all().await;

    let all_metrics_after_clear = repository.find_all(None).await.unwrap();
    assert_eq!(all_metrics_after_clear.len(), 0);
}

#[tokio::test]
async fn test_metrics_repository_count() {
    let repository = MetricsRepository::new();

    repository.clear_all().await;

    let metric1 = Metric::new(
        "CPU Usage",
        75.5,
        "%",
        "server_1",
        MetricCategory::CPU,
    ).unwrap();

    let metric2 = Metric::new(
        "Memory Usage",
        85.0,
        "%",
        "server_2",
        MetricCategory::Memory,
    ).unwrap();

    repository.create(metric1).await.unwrap();
    repository.create(metric2).await.unwrap();

    let count = repository.count().await.unwrap();
    assert_eq!(count, 2);
}

#[tokio::test]
async fn test_metrics_repository_find_critical_metrics() {
    let repository = MetricsRepository::new();

    let critical_metric = Metric::new(
        "CPU Usage",
        95.0,
        "%",
        "server_1",
        MetricCategory::CPU,
    ).unwrap();

    let normal_metric = Metric::new(
        "Memory Usage",
        50.0,
        "%",
        "server_2",
        MetricCategory::Memory,
    ).unwrap();

    repository.create(critical_metric).await.unwrap();
    repository.create(normal_metric).await.unwrap();

    let critical_metrics = repository.find_critical_metrics().await.unwrap();

    assert_eq!(critical_metrics.len(), 1);
    assert_eq!(critical_metrics[0].name, "CPU Usage");
    assert_eq!(critical_metrics[0].value, 95.0);
}