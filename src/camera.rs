// src/camera.rs

//! Defines the camera for viewing the game world.

use crate::math::Vector2D;

pub struct Camera {
    pub position: Vector2D,
    #[allow(dead_code)]
    pub velocity: Vector2D,
    pub tightness: f32,
    pub virtual_width: f32,
    pub virtual_height: f32,
    pub map_width: f32,
    pub map_height: f32,
    pub slow_zone: f32,
    pub fast_zone: f32,
    pub target_y: f32,
    #[allow(dead_code)]
    pub vertical_snap_threshold: f32,
    pub vertical_tightness: f32,
    pub camera_falling_tightness: f32,
    pub camera_falling_velocity_threshold: f32,
    pub entity_max_fall_speed: f32,
}

impl Camera {
    pub fn new(x: f32, y: f32, tightness: f32, virtual_width: f32, virtual_height: f32, map_width: f32, map_height: f32, slow_zone: f32, fast_zone: f32, vertical_snap_threshold: f32, vertical_tightness: f32, camera_falling_tightness: f32, camera_falling_velocity_threshold: f32, entity_max_fall_speed: f32) -> Self {
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
            camera_falling_tightness,
            camera_falling_velocity_threshold,
            entity_max_fall_speed,
        }
    }

    /// Updates the camera's position to center on a target with dynamic acceleration and a fast zone.
    pub fn update(&mut self, target: Vector2D, _is_grounded: bool, player_vel_y: f32) {
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

        // Vertical camera movement
        self.target_y = target.y - (self.virtual_height / 2.0);

        let vertical_tightness = if player_vel_y > self.camera_falling_velocity_threshold { // Falling fast
            let t = (player_vel_y - self.camera_falling_velocity_threshold) / (self.entity_max_fall_speed - self.camera_falling_velocity_threshold);
            let t = t.clamp(0.0, 1.0); // Ensure t is between 0 and 1
            self.vertical_tightness + t * (self.camera_falling_tightness - self.vertical_tightness) // lerp
        } else { // Jumping or on ground
            self.vertical_tightness
        };

        let delta_y = self.target_y - self.position.y;
        let move_y = delta_y * vertical_tightness;

        self.position.x += move_x;
        self.position.y += move_y;

        // Clamp camera position to map boundaries
        self.position.x = self.position.x.clamp(0.0, self.map_width - self.virtual_width);
        self.position.y = self.position.y.clamp(0.0, self.map_height - self.virtual_height);
    }
}