use ron::de::from_str;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

/// Path to the diagnostic manifest RON file
const DIAGNOSTIC_MANIFEST_PATH: &str = "/manifest/diagnostic_manifest.ron";

/// Base path to the fixers directory
const FIXERS_BASE_PATH: &str = "src/fixers";

fn to_snake_case(s: &str) -> String {
    let mut snake = String::new();
    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() {
            if i != 0 {
                snake.push('_');
            }
            for c in ch.to_lowercase() {
                snake.push(c);
            }
        } else {
            snake.push(ch);
        }
    }
    snake
}

fn to_pascal_case(s: &str) -> String {
    let mut pascal = String::new();
    let mut capitalize_next = true;
    for ch in s.chars() {
        if ch == '_' || ch == '-' {
            capitalize_next = true;
        } else if capitalize_next {
            for c in ch.to_uppercase() {
                pascal.push(c);
            }
            capitalize_next = false;
        } else {
            pascal.push(ch);
        }
    }
    pascal
}

#[derive(Debug, Deserialize)]
struct DiagnosticMetaEntry {
    kind: Option<String>,
    subkind: Option<String>,
    #[allow(dead_code)]
    step: Option<String>,
}

fn main() {
    println!("🔧 Fixer Macro Invoker Patcher Running...");

    let raw =
        fs::read_to_string(DIAGNOSTIC_MANIFEST_PATH).expect("Failed to read diagnostic manifest");
    let entries: Vec<DiagnosticMetaEntry> =
        from_str(&raw).expect("Failed to parse diagnostic manifest");

    let mut domain_macros: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();

    for entry in entries {
        if let (Some(kind), Some(subkind)) = (entry.kind, entry.subkind) {
            let subkind_pascal = to_pascal_case(&subkind) + "Fixer";
            let subkind_snake = format!("fix_{}", to_snake_case(&subkind));

            let macro_invocation = format!(
                "crate::{}_fixer!({}, {});",
                kind.to_lowercase(),
                subkind_pascal,
                subkind_snake
            );

            domain_macros
                .entry(kind.to_lowercase())
                .or_default()
                .push(macro_invocation);
        }
    }

    for (domain, macros) in domain_macros {
        let mod_rs_path = PathBuf::from(FIXERS_BASE_PATH).join(&domain).join("mod.rs");

        if !mod_rs_path.exists() {
            fs::create_dir_all(mod_rs_path.parent().unwrap())
                .expect("Failed to create fixer domain directory");
            fs::write(&mod_rs_path, "").expect("Failed to create mod.rs file");
        }

        let content = fs::read_to_string(&mod_rs_path).unwrap_or_default();
        let mut lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();

        let mut modified = false;
        for m in macros {
            if !lines.iter().any(|l| l.trim() == m) {
                lines.push(m);
                modified = true;
            }
        }

        if modified {
            lines.sort();
            let new_content = lines.join("\n") + "\n";
            fs::write(&mod_rs_path, new_content).expect("Failed to write mod.rs");
            println!("✅ Updated {}", mod_rs_path.display());
        } else {
            println!("✅ {} already up-to-date.", mod_rs_path.display());
        }
    }
}
