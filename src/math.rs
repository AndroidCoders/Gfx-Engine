// src/math.rs

//! Defines common mathematical structures used throughout the engine.

use serde::Deserialize;

/// A 2D vector with `x` and `y` components.
///
/// Used for representing positions, velocities, and other 2D quantities.
#[derive(Deserialize, Clone, Copy, Debug, Default)]
pub struct Vector2D {
    /// The x-component of the vector.
    pub x: f32,
    /// The y-component of the vector.
    pub y: f32,
}

impl Vector2D {
    /// Creates a new `Vector2D` with the given `x` and `y` components.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::math::Vector2D;
    ///
    /// let vec = Vector2D::new(10.0, 20.0);
    /// assert_eq!(vec.x, 10.0);
    /// assert_eq!(vec.y, 20.0);
    /// ```
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
