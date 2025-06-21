// src/fixers/misc/subfixes.rs

use crate::types::fix_scope::{FixScope, ScopeCategory};
use crate::types::{emotion_phase::EmotionPhase, fix_step::FixStep, fixer_context::FixContext};
use crate::utils::soulfixer_utils::apply_fix;

pub struct SubMiscFixer;

impl SubMiscFixer {
    pub fn fix_null_slots(ctx: &mut FixContext, scope: &mut FixScope) -> String {
        if ctx.emotion_phase == EmotionPhase::Frozen {
            ctx.whisper("🥶 EmotionPhase is Frozen. Skipping fix_null_slots.");
            return ctx.input.to_string();
        }

        if !scope.allows(ScopeCategory::Misc) {
            ctx.whisper("FixScope excludes Misc: skipping fix_null_slots.");
            return ctx.input.to_string();
        }

        apply_fix(
            ctx,
            "RE_NULL_SLOTS",
            "${1}null$2",
            FixStep::MiscNullSlotsFilled,
            "Filled empty slots with `null`",
        )
    }

    pub fn fix_fallback_artifacts(ctx: &mut FixContext, scope: &mut FixScope) -> String {
        if ctx.emotion_phase == EmotionPhase::Frozen {
            ctx.whisper("🥶 EmotionPhase is Frozen. Skipping fix_fallback_artifacts.");
            return ctx.input.to_string();
        }

        if !scope.allows(ScopeCategory::Misc) {
            ctx.whisper("FixScope excludes Misc: skipping fix_fallback_artifacts.");
            return ctx.input.to_string();
        }

        apply_fix(
            ctx,
            "RE_FALLBACK_ARTIFACTS",
            ", $1",
            FixStep::MiscFallbackApplied,
            "Fixed fallback GPT artifacts",
        )
    }
}
