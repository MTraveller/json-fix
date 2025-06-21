use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
struct RegexEntry {
    pattern: String,
    description: Option<String>,
    category: Option<String>,
    _engine: Option<String>,
}

fn sanitize_key(key: &str) -> String {
    let key = if key.starts_with("RE_") {
        &key[3..]
    } else {
        key
    };
    let mut out = String::from("RE_");
    out.extend(key.chars().map(|c| {
        if c.is_ascii_alphanumeric() {
            c.to_ascii_uppercase()
        } else {
            '_'
        }
    }));
    out
}

fn main() {
    let project_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("json-fixer");
    let manifest_path = project_root.join("manifest/regex_map.ron");
    let output_dir = project_root.join("src/generated_patterns/regex");
    let registry_path = project_root.join("src/generated/regex_registry.rs");
    let regex_mod_rs_path = output_dir.join("mod.rs");
    let generated_mod_rs_path = project_root.join("src/generated/mod.rs");
    let src_dir = project_root.join("src");
    let lib_rs = src_dir.join("lib.rs");
    let main_rs = src_dir.join("main.rs");

    let content = fs::read_to_string(&manifest_path).expect("Failed to read regex manifest");

    let parsed: HashMap<String, RegexEntry> =
        ron::from_str(&content).expect("Failed to parse regex manifest");

    println!("✅ Loaded {} regex entries", parsed.len());

    // Detect duplicate patterns
    let mut pattern_to_keys: HashMap<String, Vec<String>> = HashMap::new();
    for (key, entry) in &parsed {
        pattern_to_keys
            .entry(entry.pattern.clone())
            .or_default()
            .push(key.clone());
    }
    for (pattern, keys) in &pattern_to_keys {
        if keys.len() > 1 {
            println!("⚠️ Duplicate pattern detected:");
            println!("   Pattern: r\"{}\"", pattern);
            println!(
                "   Found in keys: {}",
                keys.iter()
                    .map(|k| sanitize_key(k))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
    }

    fs::create_dir_all(&output_dir).expect("Failed to create output dir");

    let mut grouped: HashMap<String, Vec<(String, RegexEntry)>> = HashMap::new();
    for (key, entry) in &parsed {
        {
            let category = entry.category.clone().unwrap_or("misc".into());
            grouped
                .entry(category)
                .or_default()
                .push((key.clone(), entry.clone()));
        }
    }

    println!("📦 Grouped into {} categories", grouped.len());

    let grouped_keys: Vec<String> = grouped.keys().cloned().collect();

    for (category, entries) in &grouped {
        let file_path = output_dir.join(format!("{}.rs", category));
        println!("📝 Writing file: {}", file_path.display());
        let mut content = String::new();

        content.push_str("// 💡 AUTO-GENERATED FILE — DO NOT EDIT\n");
        content.push_str("// ⚙️  To regenerate: cargo run --bin regex_manifest_codegen\n\n");
        content.push_str("#![allow(clippy::all)]\n\n");

        content.push_str(
            "// -----------------------------------------------------------------------------\n",
        );
        content.push_str("// ── Regex Constants ─────────────────────────────\n\n");

        content.push_str("use once_cell::sync::Lazy as LazyRegex;\nuse regex::Regex;\nuse std::collections::HashMap;\n\n");

        for (key, entry) in entries {
            let safe_key = sanitize_key(key);
            content.push_str(&format!(
                "/// {}\npub static {}: LazyRegex<Regex> = LazyRegex::new(|| Regex::new(r#\"{}\"#).unwrap());\n\n",
                entry.description.clone().unwrap_or_else(|| "".into()),
                safe_key,
                entry.pattern
            ));
        }

        content.push_str("// ── Descriptions Map ───────────────────────────\n\n");
        content.push_str("pub static REGEX_DESCRIPTIONS: LazyRegex<HashMap<&'static str, &'static str>> = LazyRegex::new(|| {\n");
        content.push_str("    HashMap::from([\n");

        for (key, entry) in entries {
            let safe_key = sanitize_key(key);
            let desc = entry.description.clone().unwrap_or_default();
            content.push_str(&format!("        (\"{}\", \"{}\"),\n", safe_key, desc));
        }

        content.push_str("    ])\n});\n");

        fs::write(file_path, content).expect("Failed to write generated file");
    }

    // --- Split mod.rs logic for categories and registry ---
    // Path for regex category modules mod.rs
    // let regex_mod_rs_path = output_dir.join("mod.rs");
    // Path for generated/mod.rs (for registry)
    // let generated_mod_rs_path = Path::new("../../json-fixer/src/generated/mod.rs");

    // --- Handle regex_mod_rs: only categories ---
    // Read existing regex_mod.rs if exists
    let existing_regex_mod_rs = if regex_mod_rs_path.exists() {
        fs::read_to_string(&regex_mod_rs_path).unwrap_or_default()
    } else {
        String::new()
    };
    let mut regex_mod_lines_set: HashSet<String> = HashSet::new();
    for line in existing_regex_mod_rs.lines() {
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            regex_mod_lines_set.insert(trimmed.to_string());
        }
    }
    // Add only category modules
    for category in &grouped_keys {
        regex_mod_lines_set.insert(format!("pub mod {};", category));
        regex_mod_lines_set.insert(format!("pub use self::{}::*;", category));
    }
    let mut regex_mod_lines: Vec<String> = regex_mod_lines_set.into_iter().collect();
    regex_mod_lines.sort();
    let mut final_regex_mod_rs = String::new();
    final_regex_mod_rs.push_str("// 💡 AUTO-GENERATED FILE — DO NOT EDIT\n");
    final_regex_mod_rs.push_str("// ⚙️  To regenerate: cargo run --bin regex_manifest_codegen\n\n");
    for line in &regex_mod_lines {
        final_regex_mod_rs.push_str(line);
        final_regex_mod_rs.push('\n');
    }
    fs::write(regex_mod_rs_path, final_regex_mod_rs).expect("Failed to write regex mod.rs");

    // --- Handle generated_mod_rs: only registry ---
    // Read existing generated/mod.rs if exists
    // let existing_generated_mod_rs = if generated_mod_rs_path.exists() {
    //     fs::read_to_string(&generated_mod_rs_path).unwrap_or_default()
    // } else {
    //     String::new()
    // };
    let existing_generated_mod_rs = if generated_mod_rs_path.exists() {
        fs::read_to_string(&generated_mod_rs_path).unwrap_or_default()
    } else {
        String::new()
    };
    let mut generated_mod_lines_set: HashSet<String> = HashSet::new();
    for line in existing_generated_mod_rs.lines() {
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            generated_mod_lines_set.insert(trimmed.to_string());
        }
    }
    // Ensure regex_registry is declared
    let regex_registry_decl = "pub mod regex_registry;";
    if !generated_mod_lines_set
        .iter()
        .any(|l| l == regex_registry_decl)
    {
        generated_mod_lines_set.insert(regex_registry_decl.to_string());
    }
    let mut generated_mod_lines: Vec<String> = generated_mod_lines_set.into_iter().collect();
    generated_mod_lines.sort();
    let mut final_generated_mod_rs = String::new();
    final_generated_mod_rs.push_str("// 💡 AUTO-GENERATED FILE — DO NOT EDIT\n");
    final_generated_mod_rs
        .push_str("// ⚙️  To regenerate: cargo run --bin regex_manifest_codegen\n\n");
    for line in &generated_mod_lines {
        final_generated_mod_rs.push_str(line);
        final_generated_mod_rs.push('\n');
    }
    fs::write(generated_mod_rs_path, final_generated_mod_rs)
        .expect("Failed to write generated/mod.rs");

    println!(
        "✅ Output complete → {} categories, {} regexes",
        grouped.len(),
        parsed.len(),
    );

    // -------------------------------------------------------------------------
    // Write regex_registry.rs mapping regex keys to fix steps and reasons
    // let registry_path = Path::new("../../json-fixer/src/generated/regex_registry.rs");
    let mut registry_content = String::new();

    registry_content.push_str("// 💡 AUTO-GENERATED FILE — DO NOT EDIT\n");
    registry_content.push_str("// ⚙️  To regenerate: cargo run --bin regex_manifest_codegen\n\n");
    registry_content.push_str("#![allow(clippy::all)]\n\n");

    registry_content.push_str(
        "// -----------------------------------------------------------------------------\n\n",
    );

    registry_content.push_str("use once_cell::sync::Lazy as LazyRegex;\n");
    registry_content.push_str("use regex::Regex;\n");
    registry_content.push_str("use std::collections::HashMap;\n");
    registry_content.push_str("use crate::generated_patterns::regex::*;\n\n");

    registry_content.push_str("pub struct RegexFixEntry {\n");
    registry_content.push_str("    pub regex: &'static Regex,\n");
    registry_content.push_str("    pub step: &'static str,\n");
    registry_content.push_str("    pub reason: &'static str,\n");
    registry_content.push_str("}\n\n");

    registry_content.push_str("pub static REGEX_FIXES: LazyRegex<HashMap<&'static str, RegexFixEntry>> = LazyRegex::new(|| {\n");
    registry_content.push_str("    HashMap::from([\n");

    for (key, entry) in &parsed {
        let safe_key = sanitize_key(key);
        let step = "UNKNOWN_STEP";
        let reason = entry.description.as_deref().unwrap_or("UNKNOWN_REASON");
        registry_content.push_str(&format!(
            "        (\"{}\", RegexFixEntry {{ regex: &{}, step: \"{}\", reason: \"{}\" }}),\n",
            safe_key, safe_key, step, reason
        ));
    }

    registry_content.push_str("    ])\n});\n");

    fs::write(registry_path, registry_content).expect("Failed to write regex_registry.rs");

    // -------------------------------------------------------------------------
    // Stub for future unused pattern warning logic
    // TODO: Implement detection and warnings for unused regex patterns

    // -------------------------------------------------------------------------
    // Check for src/lib.rs or src/main.rs and remind about pub mod generated; and pub mod generated_patterns;
    // let src_dir = Path::new("../../json-fixer/src");
    // let lib_rs = src_dir.join("lib.rs");
    // let main_rs = src_dir.join("main.rs");

    let mut remind = false;

    if lib_rs.exists() {
        let lib_content = fs::read_to_string(&lib_rs).unwrap_or_default();
        if !lib_content.contains("pub mod generated;")
            || !lib_content.contains("pub mod generated_patterns;")
        {
            remind = true;
        }
    } else if main_rs.exists() {
        let main_content = fs::read_to_string(&main_rs).unwrap_or_default();
        if !main_content.contains("pub mod generated;")
            || !main_content.contains("pub mod generated_patterns;")
        {
            remind = true;
        }
    }

    if remind {
        println!(
            "💡 Reminder: Add `pub mod generated;` and `pub mod generated_patterns;` to your lib.rs"
        );
    }

    println!(
        "✅ Done: Wrote 1 registry file, {} category modules, and updated generated/mod.rs",
        grouped.len(),
    );
    println!("💅 Tip: Run `cargo fmt` to polish generated code");
}
