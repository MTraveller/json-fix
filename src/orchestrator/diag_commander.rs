// src/orchestrator/diag_commander.rs

use crate::diagnostics::diagnoser::run_all_diagnosers;
use crate::meta::diagnostic_manifest_loader::DIAGNOSTIC_MANIFEST;
use crate::types::diagnostic_core::FixDiagnostic;
use crate::types::fix_scope::ScopeCategory;
use crate::utils::diagnostic_utils::category_label;

/// Commander for mapping diagnostics to fix scope categories.
pub struct DiagCommander;

impl DiagCommander {
    pub fn map(diags: &[FixDiagnostic]) -> Vec<ScopeCategory> {
        let mut scopes = Vec::new();

        for diag in diags {
            if let Some(entry) = DIAGNOSTIC_MANIFEST
                .iter()
                .find(|entry| entry.kind == category_label(&diag.kind))
            {
                if let Ok(scope) = entry.category.parse::<ScopeCategory>() {
                    if !scopes.contains(&scope) {
                        scopes.push(scope);
                    }
                }
            }
        }

        scopes
    }

    /// Runs all diagnosers and returns a flat list of diagnostics.
    pub fn run_all(input: &str) -> Vec<FixDiagnostic> {
        run_all_diagnosers(input)
    }
}
