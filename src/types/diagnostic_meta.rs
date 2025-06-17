#[derive(Debug, Clone, Copy)]
pub enum DiagnosticSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

impl Default for DiagnosticSeverity {
    fn default() -> Self {
        DiagnosticSeverity::Warning
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticCategory {
    Syntax,
    Structural,
    JSStyle,
    Compatibility,
    Wrapper,
}

impl Default for DiagnosticCategory {
    fn default() -> Self {
        DiagnosticCategory::Syntax
    }
}
