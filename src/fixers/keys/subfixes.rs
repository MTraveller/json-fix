// src/fixers/keys/subfixes.rs

use crate::types::{emotion_phase::EmotionPhase, fix_step::FixStep, fixer_context::FixContext};
use crate::utils::soulfixer_utils::apply_fix;

pub struct SubKeyFixer;

impl SubKeyFixer {
    pub fn fix_unquoted_keys(ctx: &mut FixContext) -> String {
        if ctx.emotion_phase == EmotionPhase::Frozen {
            ctx.whisper("🥶 EmotionPhase is Frozen. Skipping fix_unquoted_keys.");
            return ctx.input.to_string();
        }

        apply_fix(
            ctx,
            "RE_UNQUOTED_KEYS",
            "${pre}\"${key}\":",
            FixStep::KeysUnquotedFixed,
            "Added missing quotes around object keys",
        )
    }

    pub fn fix_key_traps(ctx: &mut FixContext) -> String {
        if ctx.emotion_phase == EmotionPhase::Frozen {
            ctx.whisper("🥶 EmotionPhase is Frozen. Skipping fix_key_traps.");
            return ctx.input.to_string();
        }

        apply_fix(
            ctx,
            "RE_KEY_TRAPS",
            "\"null\"$1",
            FixStep::KeysTrapResolved,
            "Resolved problematic keys replaced with 'null'",
        )
    }
}
