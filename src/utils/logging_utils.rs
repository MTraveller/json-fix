// src/utils/logging_utils.rs

use chrono::Local;

/// Prepends a timestamp to a log message (non-colored).
pub fn timestamped_log(message: &str) -> String {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    format!("[{}] {}", timestamp, message)
}

/// Prints a timestamped info log to stdout.
pub fn log_info(message: &str) {
    println!("{}", timestamped_log(message));
}

/// Prints a timestamped warning log to stdout.
pub fn log_warn(message: &str) {
    println!("⚠️  {}", timestamped_log(message));
}

/// Prints a timestamped error log to stderr.
pub fn log_error(message: &str) {
    eprintln!("❌ {}", timestamped_log(message));
}
