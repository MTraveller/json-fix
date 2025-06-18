// src/types/emotion_phase.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmotionPhase {
    Frozen,   // Too fragile — only safe fixes allowed
    Stirring, // Warming — minimal cleanup allowed
    Ready,    // Normal execution
    Overflow, // Energetic — allow advanced/generative fixes
}
