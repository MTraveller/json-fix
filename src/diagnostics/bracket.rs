// src/diagnostics/bracket.rs

#[derive(Debug, Default)]
pub struct BracketDiagnostics {
    pub has_missing_closing: bool,
    pub has_extra_closing: bool,
    pub has_unbalanced_pairs: bool,
}

pub fn analyze_brackets(json: &str) -> BracketDiagnostics {
    let mut diag = BracketDiagnostics::default();

    let mut curly = 0;
    let mut square = 0;

    for ch in json.chars() {
        match ch {
            '{' => curly += 1,
            '}' => curly -= 1,
            '[' => square += 1,
            ']' => square -= 1,
            _ => {}
        }

        if curly < 0 || square < 0 {
            diag.has_extra_closing = true;
        }
    }

    if curly > 0 || square > 0 {
        diag.has_missing_closing = true;
    }

    if diag.has_missing_closing || diag.has_extra_closing {
        diag.has_unbalanced_pairs = true;
    }

    diag
}
