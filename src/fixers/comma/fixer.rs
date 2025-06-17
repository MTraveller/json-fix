// src/fixers/comma/fixer.rs

use crate::fixers::comma::subfixes::SubCommaFixer;
use crate::types::fix_step::FixStep;

pub struct CommaFixer<'a> {
    pub input: &'a str,
    pub steps: &'a mut Vec<FixStep>,
}

impl<'a> CommaFixer<'a> {
    pub fn apply_all(&mut self) -> String {
        let mut output = self.input.to_string();

        output = SubCommaFixer::fix_double_commas(&output, self.steps);
        output = SubCommaFixer::fix_misaligned_key_value(&output, self.steps);
        output = SubCommaFixer::fix_orphaned_values(&output, self.steps);
        output = SubCommaFixer::fix_stray_commas(&output, self.steps);
        output = SubCommaFixer::fix_chained_values(&output, self.steps);
        output = SubCommaFixer::fix_missing_commas_between_pairs(&output, self.steps);

        output
    }
}
