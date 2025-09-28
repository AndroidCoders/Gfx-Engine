// src/camera.rs

//! Defines the camera for viewing the game world.

pub struct Camera {
    pub x: i32,
    pub y: i32,
}

impl Camera {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
