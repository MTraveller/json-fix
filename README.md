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
use json_fix::fix_json_syntax;

fn main() {
    let broken = r#"{ "name": "Momo, "age": 3 }"#;
    let result = fix_json_syntax(broken);

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

---

## 📊 Benchmarks

Run:

```bash
cargo bench
```

---

## 📁 Project Structure

- `src/lib.rs` – Public-facing API
- `src/fixer.rs` – Core fix logic (regex-powered)
- `examples/quick_fix.rs` – Minimal usage demo
- `tests/fixer.rs` – Real-world test case
- `benches/fix_benchmark.rs` – Criterion benchmarks

---

## ⚖️ License

MIT – In shāʾ Allāh, may it be a source of barakah for those who use and improve it.
