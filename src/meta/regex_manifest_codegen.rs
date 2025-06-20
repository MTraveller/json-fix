// src/meta/regex_manifest_codegen.rs

use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct RegexEntry {
    pattern: String,
    description: Option<String>,
    category: Option<String>,
    enabled: Option<bool>,
    engine: Option<String>,
}

fn main() {
    let manifest_path = "manifest/regex_map.ron";
    let output_dir = Path::new("src/generated_patterns");

    let content = fs::read_to_string(manifest_path).expect("Failed to read regex manifest");

    let parsed: HashMap<String, RegexEntry> =
        ron::from_str(&content).expect("Failed to parse regex manifest");

    fs::create_dir_all(output_dir).expect("Failed to create output dir");

    let mut grouped: HashMap<String, Vec<(String, RegexEntry)>> = HashMap::new();
    for (key, entry) in parsed {
        if entry.enabled.unwrap_or(true) {
            let category = entry.category.clone().unwrap_or("misc".into());
            grouped.entry(category).or_default().push((key, entry));
        }
    }

    for (category, entries) in grouped {
        let file_path = output_dir.join(format!("{}.rs", category));
        let mut content = String::new();
        content.push_str("use once_cell::sync::Lazy;\nuse regex::Regex;\n\n");

        for (key, entry) in &entries {
            content.push_str(&format!(
                "/// {}\npub static {}: Lazy<Regex> = Lazy::new(|| Regex::new(r#\"{}\"#).unwrap());\n\n",
                entry.description.clone().unwrap_or_else(|| "".into()),
                key,
                entry.pattern
            ));
        }

        content.push_str("use std::collections::HashMap;\nuse once_cell::sync::Lazy;\n\n");
        content.push_str("pub static REGEX_DESCRIPTIONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {\n");
        content.push_str("    HashMap::from([\n");

        for (key, entry) in &entries {
            let desc = entry.description.clone().unwrap_or_default();
            content.push_str(&format!("        (\"{}\", \"{}\"),\n", key, desc));
        }

        content.push_str("    ])\n});\n");

        fs::write(file_path, content).expect("Failed to write generated file");
    }
}
