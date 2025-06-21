// src/fixers/escape/subfixes.rs

use crate::generated::fix_step::FixStep;
use crate::types::emotion_phase::EmotionPhase;
use crate::types::fix_scope::{FixScope, ScopeCategory};
use crate::types::fixer_context::FixContext;
use crate::utils::soulfixer_utils::apply_fix;

pub struct SubEscapeFixer;

impl SubEscapeFixer {
    /// Removes invalid escape sequences like `\q`, `\z`, etc.
    pub fn fix_invalid_escapes(ctx: &mut FixContext, scope: &mut FixScope) -> String {
        if ctx.emotion_phase == EmotionPhase::Frozen {
            ctx.whisper("🥶 EmotionPhase is Frozen. Skipping fix_invalid_escapes.");
            return ctx.input.to_string();
        }

        if !scope.allows(ScopeCategory::Escape) {
            ctx.whisper("FixScope excludes Escape: skipping fix_invalid_escapes.");
            return ctx.input.to_string();
        }

        apply_fix(
            ctx,
            "RE_INVALID_ESCAPES",
            "",
            FixStep::EscapeInvalidRemoved,
            "Removed invalid escape sequences",
        )
    }

    /// Replaces broken or incomplete Unicode escapes like `\u12` or `\uXYZ` with `\uFFFD` (replacement char).
    pub fn fix_broken_unicode(ctx: &mut FixContext, scope: &mut FixScope) -> String {
        if ctx.emotion_phase == EmotionPhase::Frozen {
            ctx.whisper("🥶 EmotionPhase is Frozen. Skipping fix_broken_unicode.");
            return ctx.input.to_string();
        }

        if !scope.allows(ScopeCategory::Escape) {
            ctx.whisper("FixScope excludes Escape: skipping fix_broken_unicode.");
            return ctx.input.to_string();
        }

        apply_fix(
            ctx,
            "RE_BROKEN_UNICODE_ESCAPES",
            "\\uFFFD",
            FixStep::EscapeUnicodeRepaired,
            "Replaced broken unicode escapes with replacement character",
        )
    }
}
