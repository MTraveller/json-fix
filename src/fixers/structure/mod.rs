pub mod fixer;
pub mod subfixes;

use crate::fixers::structure::fixer::StructureFixer;
use crate::types::fix_step::FixStep;

/// Entry point for structure-related fixes.
pub fn fix_structure(input: &str, steps: &mut Vec<FixStep>) -> String {
    let mut fixer = StructureFixer { input, steps };
    fixer.apply_all()
}
