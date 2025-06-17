// src/fixers/keys/mod.rs

pub mod fixer;
pub mod subfixes;

use crate::fixers::keys::fixer::KeysFixer;
use crate::types::fix_step::FixStep;

/// Entry point for key-related fixes.
/// This is what the orchestrator uses to run key fixers.
pub fn fix_keys(input: &str, steps: &mut Vec<FixStep>) -> String {
    let mut fixer = KeysFixer { input, steps };
    fixer.apply_all()
}
