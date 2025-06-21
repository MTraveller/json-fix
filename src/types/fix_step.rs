// src/types/fix_step.rs
use crate::types::diagnostic_core::{DiagnosticSeverity, FixDiagnosticKind};

/// Represents a single fix step loaded from the diagnostic manifest.
#[derive(Debug, Clone)]
pub struct FixStep {
    pub id: String,
    pub label: String,
    pub kind: FixDiagnosticKind,
    pub subkind: String,
    pub severity: DiagnosticSeverity,
    pub emotion: String,
    pub tags: Vec<String>,
}

impl FixStep {
    pub fn new(id: &str) -> Self {
        FixStep {
            id: id.to_string(),
            label: id.to_string(),
            kind: FixDiagnosticKind::Other,
            subkind: String::new(),
            severity: DiagnosticSeverity::Info,
            emotion: String::new(),
            tags: vec![],
        }
    }
}
