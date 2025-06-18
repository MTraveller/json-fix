// src/fixers/markdown/mod.rs

pub mod fixer;
pub mod subfixes;

use crate::fixers::markdown::fixer::MarkdownFixer;
use crate::types::fixer_context::FixContext;

/// Entry point for markdown-related fixes.
/// Called by the orchestrator when markdown issues are detected.
pub fn fix_markdown<'a>(ctx: &mut FixContext) -> String {
    let mut fixer = MarkdownFixer { ctx };
    fixer.apply_all()
}
