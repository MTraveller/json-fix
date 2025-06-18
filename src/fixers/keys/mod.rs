// src/fixers/keys/mod.rs

pub mod fixer;
pub mod subfixes;

use crate::fixers::keys::fixer::KeysFixer;
use crate::types::fixer_context::FixContext;

/// Entry point for key-related fixes.
/// Called by the orchestrator when key issues are detected.
pub fn fix_keys<'a>(ctx: &mut FixContext) -> String {
    let mut fixer = KeysFixer { ctx };
    fixer.apply_all()
}
