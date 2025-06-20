// src/meta/diagnostic_manifest_loader.rs

use crate::types::manifest::ManifestEntry;
use once_cell::sync::Lazy;
use ron::de::from_str;
use std::fs;

pub static DIAGNOSTIC_MANIFEST: Lazy<Vec<ManifestEntry>> = Lazy::new(|| {
    let content = fs::read_to_string("manifest/diagnostic_manifest.ron")
        .expect("Failed to read diagnostic_manifest.ron");

    from_str::<Vec<ManifestEntry>>(&content).expect("Failed to parse RON diagnostic manifest")
});
