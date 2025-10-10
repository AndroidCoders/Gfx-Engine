// src/camera.rs

//! Defines the camera for viewing the game world.

use crate::math::Vector2D;

pub struct Camera {
    pub position: Vector2D,
    pub velocity: Vector2D,
    pub tightness: f32, // How tightly the camera follows the player (e.g., 0.1)
}

impl Camera {
    pub fn new(x: f32, y: f32, tightness: f32) -> Self {
        Self {
            position: Vector2D::new(x, y),
            velocity: Vector2D::default(),
            tightness,
        }
    }

    /// Updates the camera's position based on a target position using a spring-like model.
    pub fn update(&mut self, target: Vector2D) {
        // Calculate the distance vector between the camera and the target
        let distance = Vector2D::new(target.x - self.position.x, target.y - self.position.y);

        // A damping factor to prevent oscillation. A value closer to 1.0 is stiffer,
        // closer to 0.0 is looser. 0.85 is a good starting point.
        const DAMPING: f32 = 0.85;

        // Apply the spring force to the velocity
        self.velocity.x += distance.x * self.tightness;
        self.velocity.y += distance.y * self.tightness;

        // Apply damping/friction to the velocity
        self.velocity.x *= DAMPING;
        self.velocity.y *= DAMPING;

        // Update the camera's position
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
    }
}