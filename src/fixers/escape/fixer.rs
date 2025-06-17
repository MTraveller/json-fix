// src/fixers/escape/fixer.rs

use crate::fixers::escape::subfixes::SubEscapeFixer;
use crate::types::fix_step::FixStep;

pub struct EscapeFixer<'a> {
    pub input: &'a str,
    pub steps: &'a mut Vec<FixStep>,
}

impl<'a> EscapeFixer<'a> {
    pub fn apply_all(&mut self) -> String {
        let mut output = self.input.to_string();

        output = SubEscapeFixer::fix_invalid_escapes(&output, self.steps);
        output = SubEscapeFixer::fix_broken_unicode(&output, self.steps);

        output
    }
}
