// src/fixers/escape/mod.rs

pub mod fixer;
pub mod subfixes;

use crate::fixers::escape::fixer::EscapeFixer;
use crate::types::fix_step::FixStep;

/// Entry point for escape-related fixes.
/// Used by the orchestrator to run escape fixers.
pub fn fix_escapes(input: &str, steps: &mut Vec<FixStep>) -> String {
    let mut fixer = EscapeFixer { input, steps };
    fixer.apply_all()
}
