// src/orchestrator/fixer.rs

use crate::diagnostics::{analyze_all_diagnostics, FixDiagnostics};
use crate::fixers::{
    arrays::fixer::ArrayFixer, brackets::fixer::BracketFixer, comma::fixer::CommaFixer,
    escape::fixer::EscapeFixer, keys::fixer::KeysFixer, markdown::fixer::MarkdownFixer,
    misc::fixer::MiscFixer, quotes::fixer::QuoteFixer,
};
use crate::types::emotion_phase::EmotionPhase;
use crate::types::{fix_report::FixReport, fixer_context::FixContext};

#[derive(Debug)]
pub struct FixOutcome {
    owner: &'static str,
    acted: bool,
    reason: &'static str,
}

/// Represents the result of a single fixer execution
impl FixOutcome {
    fn surrendered(owner: &'static str, reason: &'static str) -> Self {
        FixOutcome {
            owner,
            acted: false,
            reason,
        }
    }

    fn executed(owner: &'static str, reason: &'static str) -> Self {
        FixOutcome {
            owner,
            acted: true,
            reason,
        }
    }
}

pub struct FixOrchestrator;

impl FixOrchestrator {
    pub fn orchestrate_fix_flow(input: &str) -> FixReport {
        let diagnostics: FixDiagnostics = analyze_all_diagnostics(input);
        let emotion_phase = EmotionPhase::Ready;
        let mut ctx = FixContext::new(input, diagnostics.clone(), emotion_phase);

        let outcomes = vec![
            Self::apply_array_fixes(&mut ctx, &diagnostics),
            Self::apply_bracket_fixes(&mut ctx, &diagnostics),
            Self::apply_comma_fixes(&mut ctx, &diagnostics),
            Self::apply_escape_fixes(&mut ctx, &diagnostics),
            Self::apply_key_fixes(&mut ctx, &diagnostics),
            Self::apply_markdown_fixes(&mut ctx, &diagnostics),
            Self::apply_quote_fixes(&mut ctx, &diagnostics),
            Self::apply_misc_fixes(&mut ctx, &diagnostics),
        ];

        FixReport {
            original: input.to_string(),
            fixed: ctx.input,
            diagnostics,
            steps: ctx.steps.clone(),
            fixer_outcomes: outcomes,
        }
    }

    fn apply_array_fixes<'a>(ctx: &mut FixContext, diagnostics: &FixDiagnostics) -> FixOutcome {
        if diagnostics.has_array_issues {
            ArrayFixer::apply(ctx);
            FixOutcome::executed("ArrayFixer", "Detected array issues")
        } else {
            FixOutcome::surrendered("ArrayFixer", "No array issues detected")
        }
    }

    fn apply_bracket_fixes<'a>(ctx: &mut FixContext, diagnostics: &FixDiagnostics) -> FixOutcome {
        if diagnostics.has_bracket_issues {
            BracketFixer::apply(ctx);
            FixOutcome::executed("BracketFixer", "Detected bracket issues")
        } else {
            FixOutcome::surrendered("BracketFixer", "No bracket issues detected")
        }
    }

    fn apply_comma_fixes<'a>(ctx: &mut FixContext, diagnostics: &FixDiagnostics) -> FixOutcome {
        if diagnostics.has_missing_commas {
            CommaFixer::apply(ctx);
            FixOutcome::executed("CommaFixer", "Detected missing commas")
        } else {
            FixOutcome::surrendered("CommaFixer", "No missing commas detected")
        }
    }

    fn apply_escape_fixes<'a>(ctx: &mut FixContext, diagnostics: &FixDiagnostics) -> FixOutcome {
        if diagnostics.has_escape_issues {
            EscapeFixer::apply(ctx);
            FixOutcome::executed("EscapeFixer", "Detected escape issues")
        } else {
            FixOutcome::surrendered("EscapeFixer", "No escape issues detected")
        }
    }

    fn apply_key_fixes<'a>(ctx: &mut FixContext, diagnostics: &FixDiagnostics) -> FixOutcome {
        if diagnostics.has_key_errors {
            KeysFixer::apply(ctx);
            FixOutcome::executed("KeysFixer", "Detected key errors")
        } else {
            FixOutcome::surrendered("KeysFixer", "No key errors detected")
        }
    }

    fn apply_markdown_fixes<'a>(ctx: &mut FixContext, diagnostics: &FixDiagnostics) -> FixOutcome {
        if diagnostics.has_markdown_wrappers {
            MarkdownFixer::apply(ctx);
            FixOutcome::executed("MarkdownFixer", "Detected markdown wrappers")
        } else {
            FixOutcome::surrendered("MarkdownFixer", "No markdown wrappers detected")
        }
    }

    fn apply_quote_fixes<'a>(ctx: &mut FixContext, diagnostics: &FixDiagnostics) -> FixOutcome {
        if diagnostics.has_quote_issues {
            QuoteFixer::apply(ctx);
            FixOutcome::executed("QuoteFixer", "Detected quote issues")
        } else {
            FixOutcome::surrendered("QuoteFixer", "No quote issues detected")
        }
    }

    fn apply_misc_fixes<'a>(ctx: &mut FixContext, diagnostics: &FixDiagnostics) -> FixOutcome {
        if diagnostics.has_misc_issues {
            MiscFixer::apply(ctx);
            FixOutcome::executed("MiscFixer", "Detected miscellaneous issues")
        } else {
            FixOutcome::surrendered("MiscFixer", "No miscellaneous issues detected")
        }
    }
}
