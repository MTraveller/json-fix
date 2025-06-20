use crate::generated::fix_step::FixStep;
use crate::types::emotion_phase::EmotionPhase;
use crate::types::fixer_context::FixContext;
use crate::utils::soulfixer_utils::apply_fix;

pub struct SubStructureFixer;

impl SubStructureFixer {
    /// Attempts to fix concatenated root JSON objects (e.g., `}{` → `},{`)
    pub fn fix_concatenated_json(ctx: &mut FixContext) -> String {
        if ctx.emotion_phase == EmotionPhase::Frozen {
            ctx.whisper("🥶 EmotionPhase is Frozen. Skipping fix_concatenated_json.");
            return ctx.input.to_string();
        }

        apply_fix(
            ctx,
            "RE_CONCATENATED_JSON_OBJECTS",
            "},{",
            FixStep::StructureConcatenatedSplit,
            "Split concatenated root JSON objects",
        )
    }

    /// Attempts to resolve orphaned braces by balancing `{` and `}`
    pub fn fix_orphaned_braces(ctx: &mut FixContext) -> String {
        if ctx.emotion_phase == EmotionPhase::Frozen {
            ctx.whisper("🥶 EmotionPhase is Frozen. Skipping fix_orphaned_braces.");
            return ctx.input.to_string();
        }
        let mut new_input = ctx.input.to_string();
        let open_count = new_input.matches('{').count();
        let close_count = new_input.matches('}').count();

        if open_count > close_count {
            new_input.push_str(&"}".repeat(open_count - close_count));
            ctx.record(FixStep::StructureOrphanedBraceResolved);
            ctx.whisper("🔧 Resolved orphaned braces imbalance");
        } else if close_count > open_count {
            new_input = "{".repeat(close_count - open_count) + &new_input;
            ctx.record(FixStep::StructureOrphanedBraceResolved);
            ctx.whisper("🔧 Resolved orphaned braces imbalance");
        }

        new_input
    }
}
