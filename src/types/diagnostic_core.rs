use crate::types::emotion_phase::EmotionPhase;

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
pub enum FixDiagnosticKind {
    Array,
    Bracket,
    Colon,
    Comma,
    Escape,
    Key,
    Markdown,
    Misc,
    Quote,
    Structure,
    JsStyle,
    Other,
}

impl Default for FixDiagnosticKind {
    fn default() -> Self {
        FixDiagnosticKind::Misc
    }
}

#[derive(Debug, Clone)]
pub struct FixDiagnostic {
    pub kind: FixDiagnosticKind,
    pub subkind: String,
    pub step: String,
    pub severity: DiagnosticSeverity,
    pub emotion: EmotionPhase,
    pub message: String,
    pub span: Option<(usize, usize)>,
    pub regex_key: String,
    pub tags: Vec<String>,
    pub enabled: bool,
}
