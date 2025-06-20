// src/types/manifest.rs

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ManifestEntry {
    pub kind: String,
    pub scope: String,
    pub step: String,
    pub label: String,
}
