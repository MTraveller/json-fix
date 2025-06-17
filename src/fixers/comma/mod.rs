// src/fixers/comma/mod.rs

pub mod fixer;
pub mod subfixes;

use crate::fixers::comma::fixer::CommaFixer;
use crate::types::fix_step::FixStep;

/// Entry point for comma-related fixes.
/// Called by the orchestrator when comma issues are detected.
pub fn fix_commas(input: &str, steps: &mut Vec<FixStep>) -> String {
    let mut fixer = CommaFixer { input, steps };
    fixer.apply_all()
}
