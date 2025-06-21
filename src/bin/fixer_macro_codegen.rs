use std::collections::HashMap;
use std::fs::{read_to_string, write};
use std::path::Path;

use ron::de::from_str;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct DiagnosticEntry {
    pub kind: String,
    pub subkind: String,
    pub step: String,
    pub label: String,
    pub category: String,
    pub severity: String,
    pub emotion: String,
    pub tags: Vec<String>,
    pub enabled: bool,
}

fn main() {
    let manifest_path = "manifest/diagnostic_manifest.ron";
    let manifest_str = read_to_string(manifest_path).expect("Failed to read diagnostic manifest");
    let entries: Vec<DiagnosticEntry> = from_str(&manifest_str).expect("Failed to parse .ron");

    let mut scoped_entries: HashMap<String, Vec<String>> = HashMap::new();

    for entry in entries {
        // Skip if step or scope is empty
        if entry.step.is_empty() || entry.category.is_empty() {
            continue;
        }

        let domain = entry
            .scope
            .to_lowercase()
            .trim_start_matches("arrays")
            .to_string(); // crude normalization
        let struct_name = format!("{}Fixer", entry.scope);
        let method_name = kind_to_method_name(&entry.kind);

        scoped_entries
            .entry(domain.clone())
            .or_default()
            .push(format!(
                "{}_fixer!({}, {});",
                domain, struct_name, method_name
            ));
    }

    for (domain, lines) in scoped_entries {
        let mod_path = format!("src/fixers/{}/mod.rs", domain);
        let content = generate_mod_rs(lines);

        if Path::new(&mod_path).exists() {
            write(&mod_path, content).expect("Failed to write mod.rs");
            println!("✅ Patched: {}", mod_path);
        } else {
            println!("⚠️ Skipped (not found): {}", mod_path);
        }
    }
}

fn kind_to_method_name(kind: &str) -> String {
    let snake = kind
        .replace("InArray", "_in_array")
        .replace("Double", "_double")
        .replace("Misaligned", "_misaligned")
        .replace("Slot", "_slot")
        .replace("Trailing", "_trailing")
        .replace("Leading", "_leading")
        .replace("Structure", "_structure")
        .replace("Comment", "_comment")
        .replace("Undefined", "_undefined")
        .replace("NaN", "_nan")
        .replace("Added", "_added")
        .replace("Replaced", "_replaced")
        .replace("Removed", "_removed")
        .replace("Concatenated", "_concatenated")
        .replace("Corrected", "_corrected")
        .replace("Unquoted", "_unquoted")
        .replace("Orphan", "_orphan")
        .replace("Balanced", "_balanced")
        .replace("Sequence", "_sequence")
        .replace("Escape", "_escape")
        .replace("Extracted", "_extracted")
        .replace("Wrapped", "_wrapped")
        .replace("Trap", "_trap")
        .to_lowercase();

    format!("fix{}", snake)
}

fn generate_mod_rs(lines: Vec<String>) -> String {
    let header = "// AUTO-GENERATED FIXERS START\n";
    let footer = "// AUTO-GENERATED FIXERS END\n";

    format!("{}{}\n{}", header, lines.join("\n"), footer)
}
