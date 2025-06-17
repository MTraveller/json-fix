// src/types/fix_report.rs

use crate::diagnostics::diagnostics::FixDiagnostics;
use crate::types::fix_step::FixStep;

#[derive(Debug)]
pub struct FixReport {
    pub original: String,
    pub fixed: String,
    pub steps: Vec<FixStep>,
    pub diagnostics: FixDiagnostics,
}

impl FixReport {
    pub fn new(
        original: String,
        fixed: String,
        steps: Vec<FixStep>,
        diagnostics: FixDiagnostics,
    ) -> Self {
        Self {
            original,
            fixed,
            steps,
            diagnostics,
        }
    }

    pub fn is_fixed(&self) -> bool {
        self.original != self.fixed
    }
}
