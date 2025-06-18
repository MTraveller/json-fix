// src/fixers/escape/mod.rs

pub mod fixer;
pub mod subfixes;

use crate::fixers::escape::fixer::EscapeFixer;
use crate::types::fixer_context::FixContext;

/// Entry point for escape-related fixes.
/// Called by the orchestrator when escape issues are detected.
pub fn fix_escapes<'a>(ctx: &mut FixContext) -> String {
    let mut fixer = EscapeFixer { ctx };
    fixer.apply_all()
}
