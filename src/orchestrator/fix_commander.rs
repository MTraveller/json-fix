// src/orchestrator/fix_commander.rs

use crate::fixers::colon::subfixes::{ColonMissingFixer, ColonMisuseFixer};
use crate::fixers::comma::subfixes::{CommaDoubleFixer, CommaMissingFixer, CommaTrailingFixer};
use crate::fixers::quote::subfixes::{QuotesCurlyFixer, QuotesSingleFixer};
use crate::fixers::Fixer;
use crate::generated::fix_step::{
    ArraysEmptySlotsFixer, ArraysLeadingCommaFixer, ArraysStructureFixer, ArraysTrailingCommaFixer,
};
use crate::types::fix_outcome::FixOutcome;
use crate::types::fix_scope::{FixScope, ScopeCategory};
use crate::types::fixer_context::FixContext;

pub struct FixCommander;

impl FixCommander {
    pub fn get_fixers_for_scope(scope: &FixScope) -> Vec<Box<dyn Fixer>> {
        let mut fixers: Vec<Box<dyn Fixer>> = Vec::new();

        for category in scope.categories() {
            match category {
                ScopeCategory::Array => {
                    fixers.push(Box::new(ArraysTrailingCommaFixer::default()));
                }
                ScopeCategory::Array => {
                    fixers.push(Box::new(ArraysLeadingCommaFixer::default()));
                }
                ScopeCategory::Array => {
                    fixers.push(Box::new(ArraysStructureFixer::default()));
                }
                ScopeCategory::Array => {
                    fixers.push(Box::new(ArraysEmptySlotsFixer::default()));
                }
                ScopeCategory::Comma => {
                    fixers.push(Box::new(CommaDoubleFixer::default()));
                }
                ScopeCategory::Comma => {
                    fixers.push(Box::new(CommaTrailingFixer::default()));
                }
                ScopeCategory::Comma => {
                    fixers.push(Box::new(CommaMissingFixer::default()));
                }
                ScopeCategory::Colon => {
                    fixers.push(Box::new(ColonMisuseFixer::default()));
                }
                ScopeCategory::Colon => {
                    fixers.push(Box::new(ColonMissingFixer::default()));
                }
                ScopeCategory::Quote => {
                    fixers.push(Box::new(QuotesCurlyFixer::default()));
                }
                ScopeCategory::Quote => {
                    fixers.push(Box::new(QuotesSingleFixer::default()));
                }
                _ => {
                    // Unhandled scopes are simply skipped for now
                }
            }
        }

        fixers
    }

    pub fn run_fixers(ctx: &mut FixContext, scope: &FixScope) -> FixOutcome {
        let fixers = Self::get_fixers_for_scope(scope);
        let mut outcome = FixOutcome::default();

        for mut fixer in fixers {
            let result = fixer.apply_all(ctx);
            outcome.merge(result);
        }

        outcome
    }
}
