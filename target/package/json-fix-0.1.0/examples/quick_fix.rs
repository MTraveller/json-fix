use json_fix::fix_json;

fn main() {
    let broken_json = r#"{"emotion": "hopeful, "score": 80}"#;
    let report = fix_json(broken_json);

    println!("✅ Fixed JSON:\n{}", report.fixed);
    println!("🔧 Steps Applied: {:?}", report.steps);
}
