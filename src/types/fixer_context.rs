// src/types/fixer_context.rs

use crate::diagnostics::diagnostics::FixDiagnostics;
use crate::types::emotion_phase::EmotionPhase;
use crate::types::fix_environment::FixEnvironment;
use crate::types::fix_step::FixStep;

#[derive(Clone)]
pub struct FixContext {
    pub input: String,
    pub diagnostics: FixDiagnostics,
    pub steps: Vec<FixStep>,
    pub emotion_phase: EmotionPhase, // “ready”, “stirring”, “frozen”, etc.
    pub log: Vec<String>,            // Whisper-Trace logs
    pub barakah_guard: bool,         // Halts risky or ambiguous fixes
    pub halted: bool,                // If paused due to BarakahGuard
    pub environment: FixEnvironment, // "dev", "release", "test"
}

impl FixContext {
    pub fn new(input: &str, diagnostics: FixDiagnostics, emotion_phase: EmotionPhase) -> Self {
        Self {
            input: input.to_string(),
            diagnostics,
            steps: Vec::new(),
            emotion_phase,
            log: Vec::new(),
            barakah_guard: false,
            halted: false,
            environment: FixEnvironment::Dev,
        }
    }

    pub fn record(&mut self, step: FixStep) {
        self.steps.push(step);
    }

    pub fn whisper(&mut self, message: &str) {
        self.log.push(message.to_string());
    }

    pub fn halt(&mut self, reason: &str) {
        self.halted = true;
        self.whisper(&format!("⏸️ Fixing halted: {}", reason));
    }
}
