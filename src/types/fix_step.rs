// src/types/fix_step.rs

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FixStep {
    CommaDoubleRemoved,
    CommaMisalignmentFixed,
    CommaOrphanedValueHandled,
    CommaStrayRemoved,
    CommaChainedValueFixed,
    CommaMissingAdded,
    CommaTrailingRemoved,

    ColonMissingFixed,
    ColonMisuseFixed,

    QuotesSingleConverted,
    QuotesCurlyNormalized,

    BracketsBalanced,
    BracketsExtraRemoved,
    BracketsMissingAdded,

    ArraysTrailingCommaFixed,
    ArraysStructureCorrected,

    EscapeInvalidRemoved,
    EscapeUnicodeRepaired,

    KeysUnquotedFixed,
    KeysTrapResolved,

    MarkdownWrapperRemoved,
    MarkdownJsonExtracted,

    MiscNullSlotsFilled,
    MiscFallbackApplied,

    StructureConcatenatedSplit,
    StructureOrphanedBraceResolved,

    JsUndefinedReplaced,
    JsNaNReplaced,
    JsCommentsRemoved,
}

impl fmt::Display for FixStep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            FixStep::CommaDoubleRemoved => "Double commas removed",
            FixStep::CommaMisalignmentFixed => "Misaligned key-value comma fixed",
            FixStep::CommaOrphanedValueHandled => "Orphaned comma value handled",
            FixStep::CommaStrayRemoved => "Stray comma removed",
            FixStep::CommaChainedValueFixed => "Comma-chained value fixed",
            FixStep::CommaMissingAdded => "Missing commas between pairs fixed",
            FixStep::CommaTrailingRemoved => "Trailing comma removed",

            FixStep::ColonMissingFixed => "Missing colons inserted",
            FixStep::ColonMisuseFixed => "Colon misuse corrected",

            FixStep::QuotesSingleConverted => "Single quotes converted",
            FixStep::QuotesCurlyNormalized => "Curly quotes normalized",

            FixStep::BracketsBalanced => "Unbalanced brackets corrected",
            FixStep::BracketsExtraRemoved => "Extra bracket removed",
            FixStep::BracketsMissingAdded => "Missing bracket added",

            FixStep::ArraysTrailingCommaFixed => "Trailing comma in array fixed",
            FixStep::ArraysStructureCorrected => "Array structure corrected",

            FixStep::EscapeInvalidRemoved => "Invalid escape removed",
            FixStep::EscapeUnicodeRepaired => "Broken Unicode escape repaired",

            FixStep::KeysUnquotedFixed => "Unquoted keys fixed",
            FixStep::KeysTrapResolved => "Key trap resolved",

            FixStep::MarkdownWrapperRemoved => "Markdown wrapper removed",
            FixStep::MarkdownJsonExtracted => "JSON extracted from markdown",

            FixStep::MiscNullSlotsFilled => "Empty/null slots filled",
            FixStep::MiscFallbackApplied => "Fallback fixer applied",

            FixStep::StructureConcatenatedSplit => "Concatenated JSON objects split",
            FixStep::StructureOrphanedBraceResolved => "Orphaned braces resolved",

            FixStep::JsUndefinedReplaced => "Replaced 'undefined' with null",
            FixStep::JsNaNReplaced => "Replaced NaN/Infinity with null",
            FixStep::JsCommentsRemoved => "JS-style comments removed",
        };
        write!(f, "{label}")
    }
}
