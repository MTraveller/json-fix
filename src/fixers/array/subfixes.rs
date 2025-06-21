// fixers/arrays/subfixes.rs

use crate::generated_patterns::regex::*;
use crate::types::fix_scope::{FixScope, ScopeCategory};
use crate::types::{emotion_phase::EmotionPhase, fix_step::FixStep, fixer_context::FixContext};
use crate::utils::soulfixer_utils::apply_fix;

pub struct SubArrayFixer;

impl SubArrayFixer {
    /// Fixes trailing commas in arrays, e.g., [1, 2, 3, ]
    pub fn fix_trailing_commas(ctx: &mut FixContext, scope: &mut FixScope) -> String {
        if ctx.emotion_phase == EmotionPhase::Frozen {
            ctx.whisper("🥶 EmotionPhase is Frozen. Skipping fix_trailing_commas.");
            return ctx.input.to_string();
        }

        if !scope.allows(ScopeCategory::Array) {
            ctx.whisper("FixScope excludes Array: skipping fix_trailing_commas.");
            return ctx.input.to_string();
        }

        apply_fix(
            ctx,
            "RE_TRAILING_COMMA_IN_ARRAY",
            "$1",
            FixStep::ArraysTrailingCommaFixed,
            "Removed trailing comma in array",
        )
    }

    /// Fixes general array structural issues (e.g., missing commas or malformed brackets)
    pub fn fix_array_structure(ctx: &mut FixContext, scope: &mut FixScope) -> String {
        if ctx.emotion_phase == EmotionPhase::Frozen {
            ctx.whisper("🥶 EmotionPhase is Frozen. Skipping fix_array_structure.");
            return ctx.input.to_string();
        }

        if !scope.allows(ScopeCategory::Array) {
            ctx.whisper("FixScope excludes Array: skipping fix_array_structure.");
            return ctx.input.to_string();
        }

        apply_fix(
            ctx,
            "RE_ARRAY_STRING_MISALIGN",
            r#"$1, $2"#,
            FixStep::ArraysStructureCorrected,
            "Corrected array element misalignment",
        )
    }

    /// Fixes empty array slots by inserting `null` values
    pub fn fix_empty_array_slots(ctx: &mut FixContext, scope: &mut FixScope) -> String {
        if ctx.emotion_phase == EmotionPhase::Frozen {
            ctx.whisper("🥶 EmotionPhase is Frozen. Skipping fix_empty_array_slots.");
            return ctx.input.to_string();
        }

        if !scope.allows(ScopeCategory::Array) {
            ctx.whisper("FixScope excludes Array: skipping fix_empty_array_slots.");
            return ctx.input.to_string();
        }

        apply_fix(
            ctx,
            "RE_EMPTY_ARRAY_SLOTS",
            "[null,",
            FixStep::ArraysEmptySlotsFilled,
            "Filled empty array slot with null",
        )
    }

    /// Fixes leading commas in arrays, e.g., [, 1, 2] -> [1, 2]
    pub fn fix_leading_commas(ctx: &mut FixContext, scope: &mut FixScope) -> String {
        if ctx.emotion_phase == EmotionPhase::Frozen {
            ctx.whisper("🥶 EmotionPhase is Frozen. Skipping fix_leading_commas.");
            return ctx.input.to_string();
        }

        if !scope.allows(ScopeCategory::Array) {
            ctx.whisper("FixScope excludes Array: skipping fix_leading_commas.");
            return ctx.input.to_string();
        }

        apply_fix(
            ctx,
            "RE_LEADING_COMMA_IN_ARRAY",
            "[",
            FixStep::ArraysLeadingCommaFixed,
            "Removed leading comma in array",
        )
    }
}
