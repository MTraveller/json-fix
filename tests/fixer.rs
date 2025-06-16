use json_fix::fix_json_syntax;

#[test]
fn test_fix_broken_json() {
    let broken = r#"{"emotion": "hopeful, "score": 80}"#;
    let report = fix_json_syntax(broken);
    assert!(serde_json::from_str::<serde_json::Value>(&report.fixed).is_ok());
}
