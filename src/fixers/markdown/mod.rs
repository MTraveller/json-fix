// src/fixers/markdown/mod.rs

pub mod fixer;
pub mod subfixes;

use crate::fixers::markdown::fixer::MarkdownFixer;
use crate::types::fix_step::FixStep;

/// Entry point for fixing markdown-related issues in JSON.
pub fn fix_markdown<'a>(input: &'a str, steps: &'a mut Vec<FixStep>) -> String {
    let mut fixer = MarkdownFixer { input, steps };
    fixer.apply_all()
}
