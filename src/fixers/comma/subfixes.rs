// src/fixers/comma/fixes.rs

use crate::types::emotion_phase::EmotionPhase;
use crate::generated::fix_step::FixStep;
use crate::types::fixer_context::FixContext;
use crate::utils::soulfixer_utils::apply_fix;

pub struct SubCommaFixer;

impl SubCommaFixer {
    pub fn fix_double_commas(ctx: &mut FixContext) -> String {
        if ctx.emotion_phase == EmotionPhase::Frozen {
            ctx.whisper("🥶 EmotionPhase is Frozen. Skipping fix_double_commas.");
            return ctx.input.to_string();
        }

        apply_fix(
            ctx,
            "RE_DOUBLE_COMMAS",
            ",",
            FixStep::CommaDoubleRemoved,
            "Removed double commas",
        )
    }

    pub fn fix_misaligned_key_value(ctx: &mut FixContext) -> String {
        if ctx.emotion_phase == EmotionPhase::Frozen {
            ctx.whisper("🥶 EmotionPhase is Frozen. Skipping fix_misaligned_key_value.");
            return ctx.input.to_string();
        }

        apply_fix(
            ctx,
            "RE_KEY_VALUE_MISALIGNED",
            ": null, \"",
            FixStep::CommaMisalignmentFixed,
            "Fixed misaligned key-value pairs",
        )
    }

    pub fn fix_orphaned_values(ctx: &mut FixContext) -> String {
        if ctx.emotion_phase == EmotionPhase::Frozen {
            ctx.whisper("🥶 EmotionPhase is Frozen. Skipping fix_orphaned_values.");
            return ctx.input.to_string();
        }

        apply_fix(
            ctx,
            "RE_ORPHANED_STRING_VALUE",
            "null",
            FixStep::CommaOrphanedValueHandled,
            "Replaced orphaned string values with null",
        )
    }

    pub fn fix_stray_commas(ctx: &mut FixContext) -> String {
        if ctx.emotion_phase == EmotionPhase::Frozen {
            ctx.whisper("🥶 EmotionPhase is Frozen. Skipping fix_stray_commas.");
            return ctx.input.to_string();
        }

        apply_fix(
            ctx,
            "RE_STRAY_COMMA_AFTER_OPENING",
            "$1",
            FixStep::CommaStrayRemoved,
            "Removed stray comma after opening",
        )
    }

    pub fn fix_chained_values(ctx: &mut FixContext) -> String {
        if ctx.emotion_phase == EmotionPhase::Frozen {
            ctx.whisper("🥶 EmotionPhase is Frozen. Skipping fix_chained_values.");
            return ctx.input.to_string();
        }

        apply_fix(
            ctx,
            "RE_CHAINED_STRING_VALUES",
            "\"chained_value\"",
            FixStep::CommaChainedValueFixed,
            "Unified chained string values",
        )
    }

    pub fn fix_missing_commas_between_pairs(ctx: &mut FixContext) -> String {
        if ctx.emotion_phase == EmotionPhase::Frozen {
            ctx.whisper("🥶 EmotionPhase is Frozen. Skipping fix_missing_commas_between_pairs.");
            return ctx.input.to_string();
        }

        apply_fix(
            ctx,
            "RE_MISSING_COMMAS_BETWEEN_PAIRS",
            "$1, $2",
            FixStep::CommaMissingAdded,
            "Inserted missing commas between key-value pairs",
        )
    }
    pub fn fix_trailing_commas(ctx: &mut FixContext) -> String {
        if ctx.emotion_phase == EmotionPhase::Frozen {
            ctx.whisper("🥶 EmotionPhase is Frozen. Skipping fix_trailing_commas.");
            return ctx.input.to_string();
        }

        apply_fix(
            ctx,
            "RE_TRAILING_COMMA_IN_ARRAY",
            "$1",
            FixStep::CommaTrailingRemoved,
            "Removed trailing comma before closing bracket/brace",
        )
    }
}
