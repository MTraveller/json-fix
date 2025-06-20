// src/diagnostics/mod.rs

pub mod array;
pub mod bracket;
pub mod colon;
pub mod comma;
pub mod diagnoser;
pub mod escape;
pub mod js_style;
pub mod key;
pub mod markdown;
pub mod misc;
pub mod quote;
pub mod structure;

pub use diagnoser::Diagnoser;
