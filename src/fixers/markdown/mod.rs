// src/fixers/markdown/mod.rs

pub mod fixer;
pub mod subfixes;

use crate::fixers::markdown::fixer::MarkdownFixer;
use crate::types::fix_scope::{FixScope, ScopeCategory};
use crate::types::fixer_context::FixContext;

/// Entry point for markdown-related fixes.
/// Called by the orchestrator when markdown issues are detected.
pub fn fix_markdown<'a>(ctx: &mut FixContext) -> String {
    let scope = FixScope::new(&ctx.input, &[ScopeCategory::Markdown], None);
    let mut fixer = MarkdownFixer { ctx, scope };
    fixer.apply_all()
}

crate::markdown_fixer!(MarkdownWrapperFixer, markdown_wrapper);
crate::markdown_fixer!(MarkdownJsonExtractFixer, markdown_json_extract);
