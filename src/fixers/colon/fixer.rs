use crate::fixers::colon::subfixes::SubColonFixer;
use crate::types::fix_step::FixStep;

pub struct ColonFixer<'a> {
    pub input: &'a str,
    pub steps: &'a mut Vec<FixStep>,
}

impl<'a> ColonFixer<'a> {
    pub fn apply_all(&mut self) -> String {
        let mut output = self.input.to_string();

        output = SubColonFixer::fix_missing_colons(&output, self.steps);
        output = SubColonFixer::fix_colon_misuse(&output, self.steps);

        output
    }
}
