// src/fixers/brackets/fixer.rs

use crate::fixers::brackets::subfixes::SubBracketFixer;
use crate::types::fix_step::FixStep;

pub struct BracketFixer<'a> {
    pub input: &'a str,
    pub steps: &'a mut Vec<FixStep>,
}

impl<'a> BracketFixer<'a> {
    pub fn apply_all(&mut self) -> String {
        let mut output = self.input.to_string();

        output = SubBracketFixer::fix_extra_closing_brace(&output, self.steps);
        output = SubBracketFixer::fix_missing_closing_brace(&output, self.steps);

        output
    }
}
