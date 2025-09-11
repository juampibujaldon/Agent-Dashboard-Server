use agent::models::alert::{Alert, AlertCondition};

#[test]
fn alert_creation_basic() {
    let a = Alert::new("s1", "cpu_usage", 90.0, AlertCondition::GreaterThan);
    assert!(a.id.is_none());
    assert_eq!(a.server_id, "s1");
    assert_eq!(a.metric_type, "cpu_usage");
    assert_eq!(a.threshold, 90.0);
    assert!(matches!(a.condition, AlertCondition::GreaterThan));
}
