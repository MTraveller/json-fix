// src/orchestrator/fixer.rs

use crate::diagnostics::{analyze_all_diagnostics, FixDiagnostics};
use crate::fixers::{
    arrays::fixer::ArrayFixer, brackets::fixer::BracketFixer, comma::fixer::CommaFixer,
    escape::fixer::EscapeFixer, keys::fixer::KeysFixer, markdown::fixer::MarkdownFixer,
    misc::fixer::MiscFixer, quotes::fixer::QuoteFixer,
};
use crate::types::{fix_report::FixReport, fix_step::FixStep};

pub fn apply_all_fixers(input: &str) -> FixReport {
    let diagnostics: FixDiagnostics = analyze_all_diagnostics(input);
    let mut steps: Vec<FixStep> = Vec::new();
    let mut output = input.to_string();

    if diagnostics.has_array_issues {
        let mut fixer = ArrayFixer {
            input: &output,
            steps: &mut steps,
        };
        output = fixer.apply_all();
    }

    if diagnostics.has_bracket_issues {
        let mut fixer = BracketFixer {
            input: &output,
            steps: &mut steps,
        };
        output = fixer.apply_all();
    }

    if diagnostics.has_missing_commas {
        let mut fixer = CommaFixer {
            input: &output,
            steps: &mut steps,
        };
        output = fixer.apply_all();
    }

    if diagnostics.has_array_issues {
        let mut fixer = EscapeFixer {
            input: &output,
            steps: &mut steps,
        };
        output = fixer.apply_all();
    }

    if diagnostics.has_key_errors {
        let mut fixer = KeysFixer {
            input: &output,
            steps: &mut steps,
        };
        output = fixer.apply_all();
    }

    if diagnostics.has_markdown_wrappers {
        let mut fixer = MarkdownFixer {
            input: &output,
            steps: &mut steps,
        };
        output = fixer.apply_all();
    }

    if diagnostics.has_quote_issues {
        let mut fixer = QuoteFixer {
            input: &output,
            steps: &mut steps,
        };
        output = fixer.apply_all();
    }

    if diagnostics.has_misc_issues {
        let mut fixer = MiscFixer {
            input: &output,
            steps: &mut steps,
        };
        output = fixer.apply_all();
    }

    FixReport {
        original: input.to_string(),
        fixed: output,
        diagnostics,
        steps,
    }
}
