// src/fixers/keys/fixer.rs

use crate::fixers::keys::subfixes::SubKeyFixer;
use crate::types::fix_step::FixStep;

pub struct KeysFixer<'a> {
    pub input: &'a str,
    pub steps: &'a mut Vec<FixStep>,
}

impl<'a> KeysFixer<'a> {
    pub fn apply_all(&mut self) -> String {
        let mut output = self.input.to_string();

        output = SubKeyFixer::fix_unquoted_keys(&output, self.steps);
        output = SubKeyFixer::fix_key_traps(&output, self.steps);

        output
    }
}
