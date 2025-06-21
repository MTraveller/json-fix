// src/fixers/escape/mod.rs

pub mod fixer;
pub mod subfixes;

use crate::fixers::escape::fixer::EscapeFixer;
use crate::types::fix_scope::{FixScope, ScopeCategory};
use crate::types::fixer_context::FixContext;

/// Entry point for escape-related fixes.
/// Called by the orchestrator when escape issues are detected.
pub fn fix_escapes<'a>(ctx: &mut FixContext) -> String {
    let scope = FixScope::new(&ctx.input, &[ScopeCategory::Escape], None);
    let mut fixer = EscapeFixer { ctx, scope };
    fixer.apply_all()
}

crate::escape_fixer!(EscapeInvalidFixer, escape_invalid);
crate::escape_fixer!(EscapeBrokenUnicodeFixer, escape_broken_unicode);
