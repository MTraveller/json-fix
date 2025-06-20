// src/types/fix_step.rs
use crate::types::diagnostic_core::{DiagnosticSeverity, FixDiagnosticKind};

/// Represents a single fix step loaded from the diagnostic manifest.
#[derive(Debug, Clone)]
pub struct FixStep {
    pub id: String,
    pub label: String,
    pub category: FixDiagnosticKind,
    pub severity: DiagnosticSeverity,
}
