use crate::types::fix_scope::{FixScope, ScopeCategory};
use crate::types::{emotion_phase::EmotionPhase, fix_step::FixStep, fixer_context::FixContext};
use crate::utils::soulfixer_utils::apply_fix;

pub struct SubColonFixer;

impl SubColonFixer {
    /// Fixes missing colons between keys and values
    pub fn fix_missing_colons(ctx: &mut FixContext, scope: &mut FixScope) -> String {
        if ctx.emotion_phase == EmotionPhase::Frozen {
            ctx.whisper("🥶 EmotionPhase is Frozen. Skipping fix_missing_colons.");
            return ctx.input.to_string();
        }

        if !scope.allows(ScopeCategory::Colon) {
            ctx.whisper("FixScope excludes Colon: skipping fix_missing_colons.");
            return ctx.input.to_string();
        }

        apply_fix(
            ctx,
            "RE_MISSING_COLON",
            r#""$1": $2"#,
            FixStep::ColonMissingFixed,
            "Inserted missing colon between key and value",
        )
    }

    /// Fixes misuse of colons (e.g., double colons)
    pub fn fix_colon_misuse(ctx: &mut FixContext, scope: &mut FixScope) -> String {
        if ctx.emotion_phase == EmotionPhase::Frozen {
            ctx.whisper("🥶 EmotionPhase is Frozen. Skipping fix_colon_misuse.");
            return ctx.input.to_string();
        }

        if !scope.allows(ScopeCategory::Colon) {
            ctx.whisper("FixScope excludes Colon: skipping fix_colon_misuse.");
            return ctx.input.to_string();
        }

        apply_fix(
            ctx,
            "RE_DOUBLE_COLON",
            ":",
            FixStep::ColonMisuseFixed,
            "Replaced double colon with single",
        )
    }
}
