// src/fixers/mod.rs

pub mod array;
pub mod bracket;
pub mod colon;
pub mod comma;
pub mod escape;
pub mod fixer;
pub mod js_style;
pub mod key;
pub mod markdown;
pub mod misc;
pub mod quote;
pub mod structure;

pub use fixer::Fixer;
