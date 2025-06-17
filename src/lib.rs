// src/lib.rs

pub mod diagnostics;
pub mod fixers;
pub mod orchestrator;
pub mod types;
pub mod utils;

/// Main entrypoint to run the full fixer pipeline.
use crate::orchestrator::fixer::apply_all_fixers;
use crate::types::fix_report::FixReport;

/// Public API: Call this with broken JSON to receive a FixReport
pub fn fix_json(input: &str) -> FixReport {
    apply_all_fixers(input)
}
