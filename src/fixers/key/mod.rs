// src/fixers/keys/mod.rs

pub mod fixer;
pub mod subfixes;

use crate::fixers::key::fixer::KeysFixer;
use crate::types::fix_scope::{FixScope, ScopeCategory};
use crate::types::fixer_context::FixContext;

/// Entry point for key-related fixes.
/// Called by the orchestrator when key issues are detected.
pub fn fix_keys<'a>(ctx: &mut FixContext) -> String {
    let scope = FixScope::new(&ctx.input, &[ScopeCategory::Key], None);
    let mut fixer = KeysFixer { ctx, scope };
    fixer.apply_all()
}

crate::key_fixer!(KeysUnquotedFixer, keys_unquoted);
crate::key_fixer!(KeysTrapFixer, keys_trap);
