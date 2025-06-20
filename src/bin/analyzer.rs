// src/bin/analyzer.rs

use json_fix::analyze_only;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 {
        fs::read_to_string(&args[1]).expect("Failed to read file")
    } else {
        eprintln!("Usage: analyzer <file>");
        return;
    };

    let diagnostics = analyze_only(&input);
    for diag in diagnostics {
        println!("[{:?}] {} at {:?}", diag.kind, diag.message, diag.span);
    }
}
