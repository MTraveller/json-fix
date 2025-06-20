// src/fixers/quotes/subfixes.rs

use crate::types::{emotion_phase::EmotionPhase, fix_step::FixStep, fixer_context::FixContext};
use crate::utils::soulfixer_utils::apply_fix;

pub struct SubQuotesFixer;

impl SubQuotesFixer {
    pub fn fix_single_quotes(ctx: &mut FixContext) -> String {
        if ctx.emotion_phase == EmotionPhase::Frozen {
            ctx.whisper("🥶 EmotionPhase is Frozen. Skipping fix_single_quotes.");
            return ctx.input.to_string();
        }

        apply_fix(
            ctx,
            "RE_SINGLE_QUOTES",
            r#""$1""#,
            FixStep::QuotesSingleConverted,
            "Converted single quotes to standard double quotes",
        )
    }

    pub fn fix_curly_quotes(ctx: &mut FixContext) -> String {
        if ctx.emotion_phase == EmotionPhase::Frozen {
            ctx.whisper("🥶 EmotionPhase is Frozen. Skipping fix_curly_quotes.");
            return ctx.input.to_string();
        }

        let mut new_output = ctx.input.to_string();
        let curly_pairs = [("“", "\""), ("”", "\""), ("‘", "'"), ("’", "'")];

        let mut changed = false;
        for (curly, ascii) in curly_pairs {
            if new_output.contains(curly) {
                new_output = new_output.replace(curly, ascii);
                changed = true;
            }
        }

        if changed {
            ctx.record(FixStep::QuotesCurlyNormalized);
            ctx.whisper("🔧 Normalized curly quotes to ASCII quotes");
        }

        new_output
    }
}
