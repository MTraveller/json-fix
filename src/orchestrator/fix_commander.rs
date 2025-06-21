// src/orchestrator/fix_commander.rs

use crate::fixers::array::{
    ArrayEmptySlotFixer, ArrayLeadingCommaFixer, ArrayStructureMalformedFixer,
    ArrayTrailingCommaFixer,
};

use crate::types::fix_outcome::FixOutcome;
use crate::types::fix_scope::{FixScope, ScopeCategory};
use crate::types::fixer_context::FixContext;

pub struct FixCommander;

impl FixCommander {
    pub fn get_fixers_for_scope(scope: &FixScope) -> Vec<fn(&mut FixContext)> {
        let mut fixers: Vec<fn(&mut FixContext)> = Vec::new();

        for category in scope.categories() {
            match category {
                ScopeCategory::Array => {
                    fixers.push(ArrayTrailingCommaFixer::fix_trailing_commas);
                    fixers.push(ArrayLeadingCommaFixer::fix_leading_commas);
                    fixers.push(ArrayStructureMalformedFixer::fix_structure);
                    fixers.push(ArrayEmptySlotFixer::fix_empty_slots);
                }
                ScopeCategory::Bracket => {
                    fixers.push(BracketsUnbalancedFixer, brackets_unbalanced);
                    fixers.push(BracketsExtraFixer, brackets_extra);
                    fixers.push(BracketsMissingFixer, brackets_missing);
                }
                _ => {
                    // Unhandled scopes are skipped
                }
            }
        }

        fixers
    }

    pub fn run_fixers(ctx: &mut FixContext, scope: &FixScope) -> FixOutcome {
        let fixers = Self::get_fixers_for_scope(scope);
        let mut outcome = FixOutcome::default();

        for fixer_fn in fixers {
            fixer_fn(ctx);
            // Assume each function mutates ctx and logs fix steps internally
        }

        outcome
    }
}
