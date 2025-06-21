use serde::Deserialize;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct FixDiagnosticManifest {
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
    let input_path = Path::new("src/meta/diagnostic_manifest.ron");
    let output_path = Path::new("src/generated/fix_step.rs");
    let manifest_str = fs::read_to_string(&input_path).expect("Failed to read manifest");
    let manifests: Vec<FixDiagnosticManifest> =
        ron::from_str(&manifest_str).expect("Failed to parse manifest");

    let mut output = String::new();

    output.push_str("// This file is auto-generated. Do not edit manually.\n\n");
    output.push_str("#[derive(Debug, Clone)]\n");
    output.push_str("pub enum FixStep {\n");
    for entry in &manifests {
        output.push_str(&format!("    {},\n", entry.step));
    }
    output.push_str("}\n\n");

    output.push_str("pub fn description_for_fix_step(step: &FixStep) -> &'static str {\n");
    output.push_str("    match step {\n");
    for entry in &manifests {
        output.push_str(&format!(
            "        FixStep::{} => \"{}\",\n",
            entry.step, entry.label
        ));
    }
    output.push_str("    }\n");
    output.push_str("}\n");

    fs::create_dir_all("generated").expect("Failed to create output directory");
    let mut file = File::create(output_path).expect("Failed to create output file");
    file.write_all(output.as_bytes())
        .expect("Failed to write output");

    use std::ffi::OsStr;

    let mod_rs_path = Path::new("src/generated/mod.rs");
    let mut mod_contents = String::new();

    for entry in fs::read_dir("src/generated").expect("Failed to read generated dir") {
        let entry = entry.expect("Invalid entry");
        let path = entry.path();
        if path.extension() == Some(OsStr::new("rs")) {
            if let Some(name) = path.file_stem().and_then(OsStr::to_str) {
                if name != "mod" {
                    mod_contents.push_str(&format!("pub mod {};\n", name));
                }
            }
        }
    }

    fs::write(mod_rs_path, mod_contents).expect("Failed to write mod.rs");
}
