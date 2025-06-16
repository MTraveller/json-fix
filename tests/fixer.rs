use json_fix::prelude::*;

#[test]
fn test_fix_broken_json() {
    let broken = r#"{"emotion": "hopeful, "score": 80}"#;
    let report = fix_json(broken);
    assert!(serde_json::from_str::<serde_json::Value>(&report.fixed).is_ok());
}
