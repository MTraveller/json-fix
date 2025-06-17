// src/types/fixer_context.rs

use crate::diagnostics::diagnostics::FixDiagnostics;
use crate::types::fix_step::FixStep;

pub struct FixContext<'a> {
    pub input: &'a str,
    pub diagnostics: FixDiagnostics,
    pub steps: Vec<FixStep>,
}

impl<'a> FixContext<'a> {
    pub fn new(input: &'a str, diagnostics: FixDiagnostics) -> Self {
        Self {
            input,
            diagnostics,
            steps: Vec::new(),
        }
    }

    pub fn record(&mut self, step: FixStep) {
        self.steps.push(step);
    }
}
