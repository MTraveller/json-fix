// src/fixers/arrays/fixer.rs

use crate::fixers::arrays::subfixes::SubArrayFixer;
use crate::types::fix_step::FixStep;

pub struct ArrayFixer<'a> {
    pub input: &'a str,
    pub steps: &'a mut Vec<FixStep>,
}

impl<'a> ArrayFixer<'a> {
    pub fn apply_all(&mut self) -> String {
        let mut output = self.input.to_string();

        output = SubArrayFixer::fix_trailing_commas(&output, self.steps);
        output = SubArrayFixer::fix_array_structure(&output, self.steps);

        output
    }
}
