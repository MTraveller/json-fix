// src/lib.rs
pub mod diagnostics;
pub mod fixers;
pub mod generated;
pub mod generated_patterns;
pub mod macros;
pub mod meta;
pub mod orchestrator;
pub mod types;
pub mod utils;

/// Main entrypoint to run the full fixer pipeline.
use crate::orchestrator::fix_runner;
use crate::types::diagnostic_core::FixDiagnostic;
use crate::types::fix_report::FixReport;

/// Public API: Call this with broken JSON to receive a FixReport
pub fn fix_json(input: &str) -> FixReport {
    fix_runner::orchestrate_fix_flow(input)
}

/// Public API: Call this with broken JSON to receive a FixReport
pub fn analyze_only(input: &str) -> Vec<FixDiagnostic> {
    use crate::diagnostics::diagnoser::run_all_diagnosers;
    run_all_diagnosers(input)
}
