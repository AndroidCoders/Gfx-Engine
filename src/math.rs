//! # Concept: Mathematical Primitives
//! 
//! This module defines the common mathematical structures used throughout the engine.
//! It provides the foundational geometry types (like 2D Vectors) required for 
//! physics calculations and spatial positioning.

use serde::Deserialize;

/// # Concept: 2D Vector
/// A pair of X and Y coordinates representing a point, velocity, or force.
#[derive(Deserialize, Clone, Copy, Debug, Default, PartialEq)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

impl Vector2D {
    /// Creates a new vector from concrete coordinate values.
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}