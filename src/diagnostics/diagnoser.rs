// src/diagnostics/diagnoser.rs

use crate::types::diagnostic_core::FixDiagnostic;

/// Trait for all modular diagnostic analyzers.
/// Each module (e.g. arrays, brackets) should implement this to return a set of fixable diagnostics.
pub trait Diagnoser {
    fn analyze(&self, input: &str) -> Vec<FixDiagnostic>;
}

use crate::diagnostics::{
    array::ArrayDiagnoser, bracket::BracketDiagnoser, colon::ColonDiagnoser, comma::CommaDiagnoser,
    escape::EscapeDiagnoser, js_style::JSStyleDiagnoser, key::KeyDiagnoser,
    markdown::MarkdownDiagnoser, misc::MiscDiagnoser, quote::QuoteDiagnoser,
    structure::StructureDiagnoser,
};

/// Runs all available diagnosers and collects diagnostics.
pub fn run_all_diagnosers(input: &str) -> Vec<FixDiagnostic> {
    let diagnosers: Vec<Box<dyn Diagnoser>> = vec![
        Box::new(ArrayDiagnoser),
        Box::new(BracketDiagnoser),
        Box::new(ColonDiagnoser),
        Box::new(CommaDiagnoser),
        Box::new(EscapeDiagnoser),
        Box::new(JSStyleDiagnoser),
        Box::new(KeyDiagnoser),
        Box::new(MarkdownDiagnoser),
        Box::new(MiscDiagnoser),
        Box::new(QuoteDiagnoser),
        Box::new(StructureDiagnoser),
    ];

    diagnosers
        .into_iter()
        .flat_map(|d| d.analyze(input))
        .collect()
}
