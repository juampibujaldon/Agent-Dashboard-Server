use agent::models::metrics::{Metric, MetricCategory};
use agent::repositories::metrics_repository::MetricsRepository;

#[tokio::test]
async fn test_repository_creation(){
    let repository= MetricsRepository::new();

    let metric = Metric::new(
        "CPU Usage",
        75.5,
        "%",
        "server_1",
        MetricCategory::CPU,
    );

    let saved_metric = repository.create(metric)
    .await.unwrap();

    assert!(saved_metric.id.is_some());
    assert_eq!(saved_metric.name, "CPU Usage");

}

#[tokio::test]
async fn to_string