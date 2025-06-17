// src/fixers/misc/mod.rs

pub mod fixer;
pub mod subfixes;

use crate::fixers::misc::fixer::MiscFixer;
use crate::types::fix_step::FixStep;

pub fn fix_misc(input: &str, steps: &mut Vec<FixStep>) -> String {
    let mut fixer = MiscFixer { input, steps };
    fixer.apply_all()
}
