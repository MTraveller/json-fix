use json_fix::fix_json;

fn main() {
    let broken_json = r#"{"emotion": "hopeful, \"score": 80}"#;

    match fix_json(broken_json) {
        Ok(fixed) => println!("✅ Fixed JSON:\n{}", fixed),
        Err(e) => eprintln!("❌ Failed to fix JSON: {}", e),
    }
}
