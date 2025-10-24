// src/camera.rs

//! Defines the camera for viewing the game world.

use crate::math::Vector2D;

pub struct Camera {
    pub position: Vector2D,
    pub velocity: Vector2D,
    pub tightness: f32,
    pub virtual_width: f32,
    pub virtual_height: f32,
    pub map_width: f32,
    pub map_height: f32,
}

impl Camera {
    pub fn new(x: f32, y: f32, tightness: f32, virtual_width: f32, virtual_height: f32, map_width: f32, map_height: f32) -> Self {
        Self {
            position: Vector2D::new(x, y),
            velocity: Vector2D::default(),
            tightness,
            virtual_width,
            virtual_height,
            map_width,
            map_height,
        }
    }

    /// Updates the camera's position to center on a target.
    pub fn update(&mut self, target: Vector2D) {
        let desired_camera_x = target.x - (self.virtual_width / 2.0);
        let desired_camera_y = target.y - (self.virtual_height / 2.0);

        let mut desired_pos = Vector2D::new(desired_camera_x, desired_camera_y);

        // Clamp desired camera position to map boundaries
        desired_pos.x = desired_pos.x.clamp(0.0, self.map_width - self.virtual_width);
        desired_pos.y = desired_pos.y.clamp(0.0, self.map_height - self.virtual_height);

        // Apply damping
        self.position.x = self.position.x + (desired_pos.x - self.position.x) * self.tightness;
        self.position.y = self.position.y + (desired_pos.y - self.position.y) * self.tightness;
    }
}