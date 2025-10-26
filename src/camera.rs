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
    pub target_y: f32,
    pub vertical_snap_threshold: f32,
    pub vertical_tightness: f32,
}

impl Camera {
    pub fn new(x: f32, y: f32, tightness: f32, virtual_width: f32, virtual_height: f32, map_width: f32, map_height: f32, slow_zone: f32, fast_zone: f32, vertical_snap_threshold: f32, vertical_tightness: f32) -> Self {
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
            target_y: y,
            vertical_snap_threshold,
            vertical_tightness,
        }
    }

    /// Updates the camera's position to center on a target with dynamic acceleration and a fast zone.
    pub fn update(&mut self, target: Vector2D, is_grounded: bool) {
        let slow_zone_x = self.virtual_width * self.slow_zone;
        let fast_zone_x = self.virtual_width * self.fast_zone;

        let camera_center_x = self.position.x + self.virtual_width / 2.0;

        let delta_x = target.x - camera_center_x;

        let mut move_x = 0.0;
        if delta_x.abs() > slow_zone_x {
            let speed_factor = if delta_x.abs() > fast_zone_x {
                1.0 // Full speed in the fast zone
            } else {
                ((delta_x.abs() - slow_zone_x) / (fast_zone_x - slow_zone_x)).powi(3)
            };
            move_x = delta_x * speed_factor * self.tightness;
        }

        if is_grounded {
            let desired_camera_y = target.y - (self.virtual_height / 2.0);
            if (desired_camera_y - self.target_y).abs() > self.vertical_snap_threshold {
                self.target_y = desired_camera_y;
            }
        }

        let delta_y = self.target_y - self.position.y;
        let move_y = delta_y * self.vertical_tightness;

        self.position.x += move_x;
        self.position.y += move_y;

        // Clamp camera position to map boundaries
        self.position.x = self.position.x.clamp(0.0, self.map_width - self.virtual_width);
        self.position.y = self.position.y.clamp(0.0, self.map_height - self.virtual_height);
    }
}