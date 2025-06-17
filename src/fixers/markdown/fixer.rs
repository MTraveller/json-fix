// src/fixers/markdown/fixer.rs

use crate::fixers::markdown::subfixes::SubMarkdownFixer;
use crate::types::fix_step::FixStep;

pub struct MarkdownFixer<'a> {
    pub input: &'a str,
    pub steps: &'a mut Vec<FixStep>,
}

impl<'a> MarkdownFixer<'a> {
    pub fn apply_all(&mut self) -> String {
        let mut output = self.input.to_string();

        output = SubMarkdownFixer::remove_markdown_wrappers(&output, self.steps);
        output = SubMarkdownFixer::extract_json_blocks(&output, self.steps);

        output
    }
}
