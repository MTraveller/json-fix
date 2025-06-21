// src/types/fix_report.rs

use crate::generated::fix_step::FixStep;
use crate::types::{diagnostic_core::FixDiagnostic, fix_outcome::FixOutcome};

#[derive(Debug)]
pub struct FixReport {
    pub original: String,
    pub fixed: String,
    pub steps: Vec<FixStep>,
    pub diagnostics: Vec<FixDiagnostic>,
    /// Outcome summary for each fixer (whether it acted and why)
    pub fixer_outcomes: Vec<FixOutcome>,
}

impl FixReport {
    pub fn new(
        original: String,
        fixed: String,
        steps: Vec<FixStep>,
        diagnostics: Vec<FixDiagnostic>,
        fixer_outcomes: Vec<FixOutcome>,
    ) -> Self {
        Self {
            original,
            fixed,
            steps,
            diagnostics,
            fixer_outcomes,
        }
    }

    pub fn is_fixed(&self) -> bool {
        self.original != self.fixed
    }
}
