// fixers/arrays/subfixes.rs

use crate::types::{emotion_phase::EmotionPhase, fix_step::FixStep, fixer_context::FixContext};
use crate::utils::regex_utils::{RE_ARRAY_STRING_MISALIGN, RE_TRAILING_COMMA_IN_ARRAY};
use crate::utils::soulfixer_utils::apply_fix;

pub struct SubArrayFixer;

impl SubArrayFixer {
    /// Fixes trailing commas in arrays, e.g., [1, 2, 3, ]
    pub fn fix_trailing_commas(ctx: &mut FixContext) -> String {
        if ctx.emotion_phase == EmotionPhase::Frozen {
            ctx.whisper("🥶 EmotionPhase is Frozen. Skipping fix_trailing_commas.");
            return ctx.input.to_string();
        }

        apply_fix(
            ctx,
            &RE_TRAILING_COMMA_IN_ARRAY,
            "$1",
            FixStep::ArraysTrailingCommaFixed,
            "Removed trailing comma in array",
        )
    }

    /// Fixes general array structural issues (e.g., missing commas or malformed brackets)
    pub fn fix_array_structure(ctx: &mut FixContext) -> String {
        if ctx.emotion_phase == EmotionPhase::Frozen {
            ctx.whisper("🥶 EmotionPhase is Frozen. Skipping fix_array_structure.");
            return ctx.input.to_string();
        }

        apply_fix(
            ctx,
            &RE_ARRAY_STRING_MISALIGN,
            r#"$1, $2"#,
            FixStep::ArraysStructureCorrected,
            "Corrected array element misalignment",
        )
    }
}
