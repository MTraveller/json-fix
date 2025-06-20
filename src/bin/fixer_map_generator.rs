use serde::Deserialize;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    let project_root = Path::new(".");
    let regex_constants = extract_regex_constants(project_root);
    let usages = map_regex_usages(project_root, &regex_constants);

    let mut output = String::new();
    output.push_str(
        "| Fix Type | Regex Used | Utility File | Fixer Domains | Used In Tests | Notes |\n",
    );
    output.push_str(
        "|----------|-------------|---------------|----------------|----------------|--------|\n",
    );

    for (name, (pattern, util_file)) in &regex_constants {
        let usage_paths = usages.get(name);
        let mut domains = vec![];
        let mut tested = "❌";

        if let Some(paths) = usage_paths {
            for path in paths {
                if path.contains("tests/") {
                    tested = "✅";
                }
                if let Some(domain) = extract_domain(path) {
                    domains.push(domain);
                }
            }
        }

        let domains_str = if domains.is_empty() {
            "❓ Unknown".to_string()
        } else {
            domains.sort();
            domains.dedup();
            domains.join(", ")
        };

        output.push_str(&format!(
            "| `{}` | `{}` | {} | `{}` | {} | |\n",
            name, pattern, util_file, domains_str, tested
        ));
    }

    fs::create_dir_all("diagnostics").unwrap();
    let mut file = File::create("diagnostics/fix_map.md").unwrap();
    file.write_all(output.as_bytes()).unwrap();

    println!("✔️ Fix map generated at diagnostics/fix_map.md");
}

fn extract_regex_constants(project_root: &Path) -> HashMap<String, (String, String)> {
    use ron::de::from_str;
    use std::collections::HashMap as StdHashMap;

    #[derive(Deserialize)]
    struct RegexEntry {
        pattern: String,
        description: Option<String>,
        category: Option<String>,
        enabled: Option<bool>,
    }

    let ron_path = project_root.join("manifest/regex_map.ron");
    let content = fs::read_to_string(ron_path).unwrap_or_default();
    let parsed: StdHashMap<String, RegexEntry> =
        from_str(&content).expect("Failed to parse regex_map.ron");

    let mut constants = HashMap::new();
    for (name, entry) in parsed {
        constants.insert(name, (entry.pattern, "regex_map.ron".to_string()));
    }

    constants
}

fn map_regex_usages(
    project_root: &Path,
    constants: &HashMap<String, (String, String)>,
) -> HashMap<String, Vec<String>> {
    let mut usage_map: HashMap<String, Vec<String>> = HashMap::new();
    for entry in WalkDir::new(project_root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().map(|ext| ext == "rs").unwrap_or(false))
    {
        let path = entry.path();
        let content = fs::read_to_string(path).unwrap_or_default();
        for name in constants.keys() {
            if content.contains(name) {
                usage_map
                    .entry(name.clone())
                    .or_default()
                    .push(path.display().to_string());
            }
        }
    }
    usage_map
}

fn extract_domain(usage_path: &str) -> Option<String> {
    usage_path
        .split('/')
        .find(|s| {
            *s == "arrays"
                || *s == "brackets"
                || *s == "colon"
                || *s == "escape"
                || *s == "js_style"
                || *s == "keys"
                || *s == "markdown"
                || *s == "misc"
                || *s == "quotes"
                || *s == "structure"
        })
        .map(|s| s.to_string())
}
