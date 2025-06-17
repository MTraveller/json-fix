// src/fixers/quotes/fixer.rs

use crate::fixers::quotes::subfixes::SubQuotesFixer;
use crate::types::fix_step::FixStep;

pub struct QuoteFixer<'a> {
    pub input: &'a str,
    pub steps: &'a mut Vec<FixStep>,
}

impl<'a> QuoteFixer<'a> {
    pub fn apply_all(&mut self) -> String {
        let mut output = self.input.to_string();

        output = SubQuotesFixer::fix_single_quotes(&output, self.steps);
        output = SubQuotesFixer::fix_curly_quotes(&output, self.steps);

        output
    }
}
