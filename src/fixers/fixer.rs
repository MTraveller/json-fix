// src/fixers/fixer.rs

use crate::types::fix_outcome::FixOutcome;
use crate::types::fixer_context::FixContext;

/// Trait for all fixers to implement a unified interface
pub trait Fixer {
    fn apply_all(&mut self, ctx: &mut FixContext) -> FixOutcome;
}
