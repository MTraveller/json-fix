// src/lib.rs
mod fixer;

pub mod prelude {
    pub use crate::fixer::{fix_json, FixReport};
}
