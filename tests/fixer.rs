use json_fix::fix_json;

#[test]
fn test_fix_broken_emotion_json() {
    let broken = r#"{"emotion": "hopeful, \"score": 80}"#;
    let fixed = fix_json(broken).expect("Should fix malformed JSON");

    assert_eq!(fixed, r#"{"emotion": "hopeful", "score": 80}"#);
}
