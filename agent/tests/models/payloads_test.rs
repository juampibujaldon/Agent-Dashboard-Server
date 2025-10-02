use agent::models::payloads::MetricPayload;

#[test]
fn metric_payload_validation_passes() {
    let payload = MetricPayload::new("srv", 50.0, 40.0, 30.0, 65.0);
    assert!(payload.validate().is_ok());
}

#[test]
fn metric_payload_validation_fails_empty_server() {
    let payload = MetricPayload::new(" ", 50.0, 40.0, 30.0, 65.0);
    assert!(payload.validate().is_err());
}

#[test]
fn metric_payload_validation_fails_out_of_range() {
    let payload = MetricPayload::new("srv", 120.0, 40.0, 30.0, 65.0);
    let err = payload.validate().unwrap_err();
    assert!(err.contains("cpu_usage"));
}
