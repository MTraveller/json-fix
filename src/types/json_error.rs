// src/types/json_error.rs

#[derive(Debug, Clone)]
pub enum JsonErrorType {
    UnexpectedToken,
    TrailingComma,
    UnclosedBracket,
    InvalidEscape,
    UnexpectedEOF,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct JsonParseError {
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub error_type: JsonErrorType,
}

impl JsonParseError {
    pub fn new(message: &str, line: usize, column: usize, error_type: JsonErrorType) -> Self {
        Self {
            message: message.to_string(),
            line,
            column,
            error_type,
        }
    }
}
