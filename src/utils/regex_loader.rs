// src/utils/regex_loader.rs

use fancy_regex::Regex as Fanex;
use once_cell::sync::Lazy;
use regex::Regex;
use ron::de::from_str;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize)]
struct RegexEntry {
    pattern: String,
    description: Option<String>,
    category: Option<String>,
    enabled: Option<bool>,
    engine: Option<String>,
}

pub static REGEX_MAP: Lazy<HashMap<String, Regex>> =
    Lazy::new(|| load_regex_map("manifest/regex_map.ron").expect("Failed to load regex map"));

pub static FANCY_REGEX_MAP: Lazy<HashMap<String, Fanex>> = Lazy::new(|| {
    load_fancy_regex_map("manifest/regex_map.ron").expect("Failed to load fancy regex map")
});

pub fn load_regex_map(path: &str) -> anyhow::Result<HashMap<String, Regex>> {
    let content = fs::read_to_string(path)?;
    let raw_map: HashMap<String, RegexEntry> = from_str(&content)?;
    let mut compiled_map = HashMap::new();

    for (key, entry) in raw_map {
        if entry.enabled.unwrap_or(true) && entry.engine.as_deref().unwrap_or("regex") == "regex" {
            let regex = Regex::new(&entry.pattern)?;
            compiled_map.insert(key, regex);
        }
    }

    Ok(compiled_map)
}

pub fn load_fancy_regex_map(path: &str) -> anyhow::Result<HashMap<String, Fanex>> {
    let content = fs::read_to_string(path)?;
    let raw_map: HashMap<String, RegexEntry> = from_str(&content)?;
    let mut compiled_map = HashMap::new();

    for (key, entry) in raw_map {
        if entry.enabled.unwrap_or(true) && entry.engine.as_deref().unwrap_or("regex") == "fancy" {
            let regex = Fanex::new(&entry.pattern)?;
            compiled_map.insert(key, regex);
        }
    }

    Ok(compiled_map)
}
