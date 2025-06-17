// src/diagnostics/diagnostics.rs

use crate::diagnostics::{
    array::ArrayDiagnostics, bracket::BracketDiagnostics, colon::ColonDiagnostics,
    comma::CommaDiagnostics, escape::EscapeDiagnostics, js_style::JSStyleDiagnostics,
    key::KeyDiagnostics, markdown::MarkdownDiagnostics, misc::MiscDiagnostics,
    quote::QuoteDiagnostics, structure::StructureDiagnostics,
};

#[derive(Debug, Default)]
pub struct FixDiagnostics {
    // Domain-specific flags
    pub array_diag: Option<ArrayDiagnostics>,
    pub bracket_diag: Option<BracketDiagnostics>,
    pub comma_diag: Option<CommaDiagnostics>,
    pub escape_diag: Option<EscapeDiagnostics>,
    pub key_diag: Option<KeyDiagnostics>,
    pub markdown_diag: Option<MarkdownDiagnostics>,
    pub quote_diag: Option<QuoteDiagnostics>,
    pub misc_diag: Option<MiscDiagnostics>,
    pub colon_diag: Option<ColonDiagnostics>,
    pub structure_diag: Option<StructureDiagnostics>,
    pub js_diag: Option<JSStyleDiagnostics>,

    // High-level fix flags
    pub has_array_issues: bool,
    pub has_bracket_issues: bool,
    pub has_missing_commas: bool,
    pub has_escape_errors: bool,
    pub has_key_errors: bool,
    pub has_markdown_wrappers: bool,
    pub has_quote_issues: bool,
    pub has_misc_issues: bool,
    pub has_colon_errors: bool,
    pub has_structure_issues: bool,
    pub has_js_issues: bool,
}

pub fn analyze_all_diagnostics(input: &str) -> FixDiagnostics {
    let array_diag = crate::diagnostics::array::analyze_arrays(input);
    let bracket_diag = crate::diagnostics::bracket::analyze_brackets(input);
    let comma_diag = crate::diagnostics::comma::analyze_commas(input);
    let escape_diag = crate::diagnostics::escape::analyze_escapes(input);
    let key_diag = crate::diagnostics::key::analyze_keys(input);
    let markdown_diag = crate::diagnostics::markdown::analyze_markdown(input);
    let quote_diag = crate::diagnostics::quote::analyze_quotes(input);
    let misc_diag = crate::diagnostics::misc::analyze_misc(input);

    let colon_diag = crate::diagnostics::colon::analyze_colons(input);
    let structure_diag = crate::diagnostics::structure::analyze_structure(input);
    let js_diag = crate::diagnostics::js_style::analyze_js_styles(input);

    FixDiagnostics {
        has_array_issues: array_diag.has_trailing_commas
            || array_diag.has_malformed_nesting
            || array_diag.has_empty_array_slots,
        has_bracket_issues: bracket_diag.has_missing_closing
            || bracket_diag.has_extra_closing
            || bracket_diag.has_unbalanced_pairs,
        has_missing_commas: comma_diag.has_double_commas
            || comma_diag.has_key_value_misalignment
            || comma_diag.has_orphaned_values
            || comma_diag.has_stray_commas
            || comma_diag.has_comma_value_chaining,
        has_escape_errors: escape_diag.has_invalid_escape || escape_diag.has_broken_unicode,
        has_key_errors: key_diag.has_unquoted_keys || key_diag.has_key_traps,
        has_markdown_wrappers: markdown_diag.has_markdown_wrappers
            || markdown_diag.has_embedded_json,
        has_quote_issues: quote_diag.has_single_quotes
            || quote_diag.has_curly_quotes
            || quote_diag.has_smart_quotes,
        has_misc_issues: misc_diag.has_fallbacks || misc_diag.has_null_slots,
        has_colon_errors: colon_diag.has_missing_colons || colon_diag.has_colon_misuse,
        has_structure_issues: structure_diag.has_concatenated_json
            || structure_diag.has_orphaned_braces,
        has_js_issues: js_diag.has_undefined || js_diag.has_nan || js_diag.has_comments,

        array_diag: Some(array_diag),
        bracket_diag: Some(bracket_diag),
        comma_diag: Some(comma_diag),
        escape_diag: Some(escape_diag),
        key_diag: Some(key_diag),
        markdown_diag: Some(markdown_diag),
        quote_diag: Some(quote_diag),
        misc_diag: Some(misc_diag),
        colon_diag: Some(colon_diag),
        structure_diag: Some(structure_diag),
        js_diag: Some(js_diag),
    }
}
