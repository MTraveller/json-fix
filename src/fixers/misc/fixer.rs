// src/fixers/misc/fixer.rs

use crate::fixers::misc::subfixes::SubMiscFixer;
use crate::types::fix_step::FixStep;

pub struct MiscFixer<'a> {
    pub input: &'a str,
    pub steps: &'a mut Vec<FixStep>,
}

impl<'a> MiscFixer<'a> {
    pub fn apply_all(&mut self) -> String {
        let mut output = self.input.to_string();

        output = SubMiscFixer::fix_null_slots(&output, self.steps);
        output = SubMiscFixer::fix_fallback_artifacts(&output, self.steps);

        output
    }
}
