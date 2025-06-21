// src/types/manifest.rs

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ManifestEntry {
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
