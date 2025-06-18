// src/fixers/misc/mod.rs

pub mod fixer;
pub mod subfixes;

use crate::fixers::misc::fixer::MiscFixer;
use crate::types::fixer_context::FixContext;

/// Entry point for miscellaneous fixes.
/// Called by the orchestrator when miscellaneous issues are detected.
pub fn fix_misc<'a>(ctx: &mut FixContext) -> String {
    let mut fixer = MiscFixer { ctx };
    fixer.apply_all()
}
