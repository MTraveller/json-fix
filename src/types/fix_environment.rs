//src/types/fix_environment.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FixEnvironment {
    Dev,     // Development/debug — more logs, less fear
    Release, // Production — less verbose, more caution
    Test,    // Test mode — stricter assertions
}
