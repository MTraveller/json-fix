// src/fixers/brackets/mod.rs

pub mod fixer;
pub mod subfixes;

use crate::fixers::brackets::fixer::BracketFixer;
use crate::types::fix_step::FixStep;

/// Entry point for bracket-related fixes.
/// Called by the orchestrator when bracket issues are detected.
pub fn fix_brackets(input: &str, steps: &mut Vec<FixStep>) -> String {
    let mut fixer = BracketFixer { input, steps };
    fixer.apply_all()
}
