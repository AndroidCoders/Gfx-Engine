// src/math.rs

//! Defines common mathematical structures.

#[derive(Clone, Copy, Debug, Default)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

impl Vector2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
