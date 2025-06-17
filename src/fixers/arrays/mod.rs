// src/fixers/arrays/mod.rs

pub mod fixer;
pub mod subfixes;

use crate::fixers::arrays::fixer::ArrayFixer;
use crate::types::fix_step::FixStep;

/// Entry point for array-related fixes.
/// This delegates to ArrayFixer with full step tracking.
pub fn fix_arrays(input: &str, steps: &mut Vec<FixStep>) -> String {
    let mut fixer = ArrayFixer { input, steps };
    fixer.apply_all()
}
