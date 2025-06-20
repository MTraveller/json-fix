[![Crates.io](https://img.shields.io/crates/v/json-fix.svg)](https://crates.io/crates/json-fix)
[![Docs.rs](https://docs.rs/json-fix/badge.svg)](https://docs.rs/json-fix)

# 🛠️ json-fix

**json-fix** is a blazing-fast, Rust-powered JSON repair library built for resilient data pipelines, GPT outputs, web scraping tools, and any system where malformed JSON sneaks in.

It detects and fixes broken JSON strings — from misescaped quotes to missing commas — using a curated sequence of regex-powered healing steps. Ideal for both CLI tools and backend services.

---

## 🚀 Features

- ✅ Fixes unescaped or invalid quote issues
- ✅ Repairs trailing commas, missing brackets, and embedded key-value bugs
- ✅ Pure Rust — no unsafe, no dependencies outside `fancy-regex`
- ✅ Supports logs for each fix step
- ✅ Battle-tested against AI-generated JSON errors
- ✅ Easily embeddable as a library or CLI tool

---

## 🧪 Example

```rust
use json_fix::fix_json;

fn main() {
    let broken = r#"{ "name": "Momo, "age": 3 }"#;
    let result = fix_json(broken);

    if result.fixed != broken {
        println!("✅ Fixed JSON:\n{}", result.fixed);
    } else {
        println!("⚠️ No changes made.");
    }
}
```

---

## 📂 Usage

### Install:

```bash
cargo add json-fix
```

### As a library:

```rust
let result = fix_json_syntax(broken);
```

### From CLI:

```bash
cargo run --example quick_fix
```

```bash
# Optional: regenerate regex constants from the manifest
cargo run --bin regex_manifest_codegen
```

---

## 📁 Project Structure

- `src/lib.rs` – Public API entrypoint
- `src/orchestrator/` – Full diagnostic → scope → fixer execution pipeline
- `src/diagnostics/` – Modular diagnosers powered by `FixDiagnostic` output
- `src/fixers/` – Trait-based modular fixers with scoped `FixContext`
- `src/meta/regex_manifest_codegen.rs` – Build tool: generates constants from manifest
- `manifest/regex_map.ron` – One source of truth for regex patterns
- `src/generated_patterns/` – Auto-generated constants (`Lazy<Regex>`) per category
- `tests/` – Real-world fixer + diagnoser test suite
- `benches/` – Criterion benchmarks

## 📦 Manifest-Powered Regex System

Regexes are declared once in `manifest/regex_map.ron`, then compiled into fast, type-safe constants by running:

```bash
cargo run --bin regex_manifest_codegen
```

This ensures:

- ✅ One source of truth
- ✅ No runtime string-key lookups
- ✅ All patterns are testable, traceable, and Fitrah-aligned
