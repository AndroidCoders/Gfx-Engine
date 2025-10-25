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
    pub slow_zone: f32,
    pub fast_zone: f32,
}

impl Camera {
    pub fn new(x: f32, y: f32, tightness: f32, virtual_width: f32, virtual_height: f32, map_width: f32, map_height: f32, slow_zone: f32, fast_zone: f32) -> Self {
        Self {
            position: Vector2D::new(x, y),
            velocity: Vector2D::default(),
            tightness,
            virtual_width,
            virtual_height,
            map_width,
            map_height,
            slow_zone,
            fast_zone,
        }
    }

    /// Updates the camera's position to center on a target with dynamic acceleration and a fast zone.
    pub fn update(&mut self, target: Vector2D) {
        let slow_zone_x = self.virtual_width * self.slow_zone;
        let slow_zone_y = self.virtual_height * self.slow_zone;
        let fast_zone_x = self.virtual_width * self.fast_zone;
        let fast_zone_y = self.virtual_height * self.fast_zone;

        let camera_center_x = self.position.x + self.virtual_width / 2.0;
        let camera_center_y = self.position.y + self.virtual_height / 2.0;

        let delta_x = target.x - camera_center_x;
        let delta_y = target.y - camera_center_y;

        let mut move_x = 0.0;
        if delta_x.abs() > slow_zone_x {
            let speed_factor = if delta_x.abs() > fast_zone_x {
                1.0 // Full speed in the fast zone
            } else {
                ((delta_x.abs() - slow_zone_x) / (fast_zone_x - slow_zone_x)).powi(3)
            };
            move_x = delta_x * speed_factor * self.tightness;
        }

        let mut move_y = 0.0;
        if delta_y.abs() > slow_zone_y {
            let speed_factor = if delta_y.abs() > fast_zone_y {
                1.0 // Full speed in the fast zone
            } else {
                ((delta_y.abs() - slow_zone_y) / (fast_zone_y - slow_zone_y)).powi(3)
            };
            move_y = delta_y * speed_factor * self.tightness;
        }

        self.position.x += move_x;
        self.position.y += move_y;

        // Clamp camera position to map boundaries
        self.position.x = self.position.x.clamp(0.0, self.map_width - self.virtual_width);
        self.position.y = self.position.y.clamp(0.0, self.map_height - self.virtual_height);
    }
}