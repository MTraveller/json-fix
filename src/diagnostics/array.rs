// src/diagnostics/array.rs

#[derive(Debug, Default)]
pub struct ArrayDiagnostics {
    pub has_trailing_commas: bool,
    pub has_malformed_nesting: bool,
    pub has_empty_array_slots: bool,
}

pub fn analyze_arrays(json: &str) -> ArrayDiagnostics {
    let mut diag = ArrayDiagnostics::default();

    if json.contains("[,") || json.contains(",]") {
        diag.has_trailing_commas = true;
    }

    if json.contains("[][]") || json.contains("[[]") || json.contains("[]]") {
        diag.has_malformed_nesting = true;
    }

    if json.contains("[, ,") || json.contains("[null, ,") {
        diag.has_empty_array_slots = true;
    }

    diag
}
