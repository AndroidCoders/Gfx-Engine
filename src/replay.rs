//! # Concept: Input Serialization (Replays)
//! 
//! This module provides the mechanism for recording and playing back 
//! gameplay sessions. It serializes the stream of Input State facts 
//! to disk, enabling deterministic "Attract Mode" and bug reproduction.

use serde::{Serialize, Deserialize};
use crate::input::InputAction;

/// A single snapshot of all pressed actions at a specific simulation tick.
#[derive(Serialize, Deserialize, Clone)]
pub struct InputFrame {
    pub tick: u64,
    pub pressed_actions: Vec<InputAction>,
}

/// A collection of input frames representing a complete gameplay session.
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Replay {
    pub seed: u64,
    pub frames: Vec<InputFrame>,
}

impl Replay {
    #[allow(dead_code)]
    pub fn new(seed: u64) -> Self {
        Self { seed, frames: Vec::new() }
    }

    /// Serializes the current replay buffer to a JSON file on disk.
    #[allow(dead_code)]
    pub fn save(&self, name: &str) -> Result<(), String> {
        // 1. Transform the struct into a human-readable JSON string.
        let json = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        // 2. Write the resulting bytes to the assets/replays directory.
        std::fs::write(format!("assets/replays/{}.replay", name), json).map_err(|e| e.to_string())
    }

    /// Deserializes a replay file from disk into a memory buffer.
    pub fn load(name: &str) -> Result<Self, String> {
        // 1. Read the raw bytes from the specified file.
        let content = std::fs::read_to_string(format!("assets/replays/{}.replay", name)).map_err(|e| e.to_string())?;
        // 2. Parse the JSON back into a Replay struct.
        serde_json::from_str(&content).map_err(|e| e.to_string())
    }
}