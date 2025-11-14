// src/camera.rs

//! This module defines the `Camera` struct, which is responsible for controlling the
//! game's viewpoint.
//! 
//! The camera smoothly follows a target (usually the player) and implements features
//! like a "slow zone" to prevent jitter and a "fast zone" to keep the target on screen.

use crate::math::Vector2D;
use crate::ecs::component::Direction;

/// Holds the state for the game's camera and implements its movement logic.
///
/// The camera is designed to smoothly follow a target (usually the player)
/// with features inspired by classic platformers, such as look-ahead,
/// platform snapping, and different movement zones.
pub struct Camera {
    /// The current top-left position of the camera in world coordinates.
    pub position: Vector2D,
    /// The camera's current velocity (not currently used, but kept for future physics-based movement).
    #[allow(dead_code)]
    pub velocity: Vector2D,
    /// The general tightness of the camera's follow-movement (a value between 0.0 and 1.0).
    pub tightness: f32,
    /// The width of the camera's viewport in world units.
    pub virtual_width: f32,
    /// The height of the camera's viewport in world units.
    pub virtual_height: f32,
    /// The total width of the level map in world units, for clamping.
    pub map_width: f32,
    /// The total height of the level map in world units, for clamping.
    pub map_height: f32,
    /// The percentage of the screen where the player can move without the camera following.
    pub slow_zone: f32,
    /// The percentage of the screen edge that triggers a faster camera follow.
    pub fast_zone: f32,
    /// The camera's target Y-position, which is only updated when the player is on the ground.
    pub locked_y_position: f32,
    /// The velocity threshold for the camera to snap vertically (not currently used).
    #[allow(dead_code)]
    pub vertical_snap_threshold: f32,
    /// The tightness of the camera's vertical movement.
    pub vertical_tightness: f32,
    /// A tighter vertical follow setting used when the player is falling fast.
    pub camera_falling_tightness: f32,
    /// The downward velocity at which the `camera_falling_tightness` is triggered.
    pub camera_falling_velocity_threshold: f32,
    /// The maximum fall speed of an entity, used for calculating falling tightness.
    pub entity_max_fall_speed: f32,
    /// The distance the camera looks ahead of the player in the direction of movement.
    pub lookahead_distance: f32,
    /// The current smoothed lookahead offset.
    pub current_lookahead_offset: f32,
}

impl Camera {
    /// Creates a new `Camera`.
    #[allow(clippy::too_many_arguments)]
    pub fn new(x: f32, y: f32, tightness: f32, virtual_width: f32, virtual_height: f32, map_width: f32, map_height: f32, slow_zone: f32, fast_zone: f32, vertical_snap_threshold: f32, vertical_tightness: f32, camera_falling_tightness: f32, camera_falling_velocity_threshold: f32, entity_max_fall_speed: f32, lookahead_distance: f32) -> Self {
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
            locked_y_position: y,
            vertical_snap_threshold,
            vertical_tightness,
            camera_falling_tightness,
            camera_falling_velocity_threshold,
            entity_max_fall_speed,
            lookahead_distance,
            current_lookahead_offset: 0.0,
        }
    }

    /// Updates the camera's position to center on a target with dynamic acceleration and a fast zone.
    pub fn update(&mut self, target: Vector2D, is_grounded: bool, player_vel_y: f32, player_direction: Direction) {
        let slow_zone_x = self.virtual_width * self.slow_zone;
        let fast_zone_x = self.virtual_width * self.fast_zone;

        let target_lookahead_offset = if player_direction == Direction::Right {
            self.lookahead_distance
        } else {
            -self.lookahead_distance
        };

        // Smoothly interpolate the lookahead offset to prevent jumping
        self.current_lookahead_offset += (target_lookahead_offset - self.current_lookahead_offset) * 0.1;

        let camera_center_x = self.position.x + self.virtual_width / 2.0;

        // The delta is now between the player's position plus the smoothed lookahead, and the camera's center
        let delta_x = (target.x + self.current_lookahead_offset) - camera_center_x;

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
        if is_grounded {
            self.locked_y_position = target.y - (self.virtual_height / 2.0);
        }

        let vertical_tightness = if player_vel_y > self.camera_falling_velocity_threshold { // Falling fast
            let t = (player_vel_y - self.camera_falling_velocity_threshold) / (self.entity_max_fall_speed - self.camera_falling_velocity_threshold);
            let t = t.clamp(0.0, 1.0); // Ensure t is between 0 and 1
            self.vertical_tightness + t * (self.camera_falling_tightness - self.vertical_tightness) // lerp
        } else { // Jumping or on ground
            self.vertical_tightness
        };

        let delta_y = self.locked_y_position - self.position.y;
        let move_y = delta_y * vertical_tightness;

        self.position.x += move_x;
        self.position.y += move_y;

        // Clamp camera position to map boundaries
        self.position.x = self.position.x.clamp(0.0, self.map_width - self.virtual_width);
        self.position.y = self.position.y.clamp(0.0, self.map_height - self.virtual_height);
    }

    /// Instantly snaps the camera to a new target position.
    pub fn snap_to(&mut self, target: Vector2D) {
        self.position.x = (target.x - self.virtual_width / 2.0).clamp(0.0, self.map_width - self.virtual_width);
        self.position.y = (target.y - self.virtual_height / 2.0).clamp(0.0, self.map_height - self.virtual_height);
        self.velocity = Vector2D::default();
    }
}