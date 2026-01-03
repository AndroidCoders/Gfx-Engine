//! # Concept: Configuration Infrastructure
//! 
//! This module provides the data-driven foundation for the engine.
//! It handles the loading of TOML settings and defines the global 
//! scaling constants required for the 1:1 pixel coordinate system.

/// The multiplier used to project the logical game world (Retro) to the screen (HD).
pub const RENDER_SCALE_FACTOR: f32 = 4.0;

pub mod core;
pub mod game;

// Re-export core configs
pub use self::core::*;

// Re-export game configs
pub use self::game::*;