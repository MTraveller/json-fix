pub mod fixer;
pub mod subfixes;

use crate::fixers::colon::fixer::ColonFixer;
use crate::types::fix_step::FixStep;

/// Entry point for colon-related fixes.
pub fn fix_colons(input: &str, steps: &mut Vec<FixStep>) -> String {
    let mut fixer = ColonFixer { input, steps };
    fixer.apply_all()
}
