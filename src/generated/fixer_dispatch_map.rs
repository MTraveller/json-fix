use crate::fixer::Fixer;
use crate::fixers::array::subfixes::{
    ArraysEmptySlotsFixer, ArraysLeadingCommaFixer, ArraysStructureFixer, ArraysTrailingCommaFixer,
};
use crate::types::fix_step::FixStep;

/// Returns the corresponding fixer instance for a given `FixStep`, if available.
pub fn create_fixer(step: &FixStep) -> Option<Box<dyn Fixer>> {
    match step {
        FixStep::ArraySlotRemoved => Some(Box::new(ArraysEmptySlotsFixer)),
        FixStep::LeadingCommaInArrayFixed => Some(Box::new(ArraysLeadingCommaFixer)),
        FixStep::StructureConcatenated => Some(Box::new(ArraysStructureFixer)),
        FixStep::TrailingCommaInArrayFixed => Some(Box::new(ArraysTrailingCommaFixer)),
        _ => None,
    }
}
