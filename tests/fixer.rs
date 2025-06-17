// tests/fixer.rs

use json_fix::orchestrator::fixer::apply_all_fixers;
use serde_json::Value;

#[test]
fn test_fix_basic_json() {
    let broken_json = r#"
    {
        "key1": "value1",
        "key2": "value2",
    }
    "#;

    let expected_fixed_json = r#"
    {
        "key1": "value1",
        "key2": "value2"
    }
    "#;

    let fix_report = apply_all_fixers(broken_json);

    // The fixed output should be valid JSON parsable string
    let fixed_value: Value =
        serde_json::from_str(&fix_report.fixed).expect("Fixed JSON should parse");

    // The expected fixed JSON parsed for comparison
    let expected_value: Value =
        serde_json::from_str(expected_fixed_json).expect("Expected JSON should parse");

    assert_eq!(fixed_value, expected_value);

    // Optionally assert that some fixes were recorded
    assert!(
        !fix_report.steps.is_empty(),
        "Fix steps should not be empty"
    );
}

#[test]
fn test_fix_missing_commas() {
    let broken_json = r#"{"a": 1 "b": 2}"#;
    let fix_report = apply_all_fixers(broken_json);
    assert!(fix_report.steps.len() > 0, "Fix steps should be detected");
    // Add more asserts depending on your expectations
}

#[test]
fn test_no_changes_for_valid_json() {
    let valid_json = r#"{"foo": "bar", "baz": 42}"#;
    let fix_report = apply_all_fixers(valid_json);
    assert_eq!(fix_report.fixed, valid_json, "Valid JSON should not change");
    assert!(fix_report.steps.is_empty(), "No fix steps for valid JSON");
}
