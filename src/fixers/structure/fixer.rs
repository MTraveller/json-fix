use crate::fixers::structure::subfixes::SubStructureFixer;
use crate::types::fix_step::FixStep;

pub struct StructureFixer<'a> {
    pub input: &'a str,
    pub steps: &'a mut Vec<FixStep>,
}

impl<'a> StructureFixer<'a> {
    pub fn apply_all(&mut self) -> String {
        let mut output = self.input.to_string();

        output = SubStructureFixer::fix_concatenated_json(&output, self.steps);
        output = SubStructureFixer::fix_orphaned_braces(&output, self.steps);

        output
    }
}
