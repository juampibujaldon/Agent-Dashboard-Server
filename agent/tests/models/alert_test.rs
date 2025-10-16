use agent::models::alert::{Alert, AlertCondition};
use chrono::{Utc};

#[test]
fn alert_creation_basic() {
    let a = Alert::new("s1", "cpu_usage", 90.0, AlertCondition::GreaterThan).unwrap();
    assert!(a.id.is_none());
    assert_eq!(a.server_id, "s1");
    assert_eq!(a.metric_type, "cpu_usage");
    assert_eq!(a.threshold, 90.0);
    assert!(matches!(a.condition, AlertCondition::GreaterThan));
    assert!(a.is_active);
}

#[test]
fn alert_with_id() {
    let alert = Alert::new("s1", "cpu_usage", 90.0, AlertCondition::GreaterThan)
        .unwrap()
        .with_id("alert_123");
    assert_eq!(alert.id, Some("alert_123".to_string()));
}

#[test]
fn alert_timestamp_is_set() {
    let before = Utc::now();
    let alert = Alert::new("s1", "cpu_usage", 90.0, AlertCondition::GreaterThan).unwrap();
    let after = Utc::now();

    assert!(alert.created_at >= before);
    assert!(alert.created_at <= after);
}

#[test]
fn alert_deactivate() {
    let mut alert = Alert::new("s1", "cpu_usage", 90.0, AlertCondition::GreaterThan).unwrap();
    assert!(alert.is_active);
    
    alert.deactivate();
    assert!(!alert.is_active);
}

#[test]
fn alert_activate() {
    let mut alert = Alert::new("s1", "cpu_usage", 90.0, AlertCondition::GreaterThan).unwrap();
    alert.deactivate();
    assert!(!alert.is_active);
    
    alert.activate();
    assert!(alert.is_active);
}

#[test]
fn alert_should_trigger_greater_than() {
    let alert = Alert::new("s1", "cpu_usage", 90.0, AlertCondition::GreaterThan).unwrap();
    
    assert!(!alert.should_trigger(89.0)); 
    assert!(!alert.should_trigger(90.0)); 
    assert!(alert.should_trigger(91.0));  
}

#[test]
fn alert_should_trigger_greater_or_equal() {
    let alert = Alert::new("s1", "cpu_usage", 90.0, AlertCondition::GreaterOrEqual).unwrap();
    
    assert!(!alert.should_trigger(89.0)); 
    assert!(alert.should_trigger(90.0));  
    assert!(alert.should_trigger(91.0));  
}

#[test]
fn alert_should_trigger_less_than() {
    let alert = Alert::new("s1", "cpu_usage", 10.0, AlertCondition::LessThan).unwrap();
    
    assert!(alert.should_trigger(9.0));  
    assert!(!alert.should_trigger(10.0)); 
    assert!(!alert.should_trigger(11.0)); 
}

#[test]
fn alert_should_trigger_less_or_equal() {
    let alert = Alert::new("s1", "cpu_usage", 10.0, AlertCondition::LessOrEqual).unwrap();
    
    assert!(alert.should_trigger(9.0));  
    assert!(alert.should_trigger(10.0));  
    assert!(!alert.should_trigger(11.0)); 
}

#[test]
fn alert_inactive_should_not_trigger() {
    let mut alert = Alert::new("s1", "cpu_usage", 90.0, AlertCondition::GreaterThan).unwrap();
    alert.deactivate();
    
    assert!(!alert.should_trigger(95.0));
}

#[test]
fn alert_validation_empty_server_id() {
    let result = Alert::new("", "cpu_usage", 90.0, AlertCondition::GreaterThan);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), agent::AppError::Validation(_)));
}

#[test]
fn alert_validation_empty_metric_type() {
    let result = Alert::new("s1", "", 90.0, AlertCondition::GreaterThan);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), agent::AppError::Validation(_)));
}

#[test]
fn alert_validation_negative_threshold() {
    let result = Alert::new("s1", "cpu_usage", -5.0, AlertCondition::GreaterThan);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), agent::AppError::Validation(_)));
}

#[test]
fn alert_validation_nan_threshold() {
    let result = Alert::new("s1", "cpu_usage", f64::NAN, AlertCondition::GreaterThan);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), agent::AppError::Validation(_)));
}

#[test]
fn alert_validation_infinite_threshold() {
    let result = Alert::new("s1", "cpu_usage", f64::INFINITY, AlertCondition::GreaterThan);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), agent::AppError::Validation(_)));
}
