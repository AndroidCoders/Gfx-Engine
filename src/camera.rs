//! # Concept: Viewport Control
//! 
//! This module is the authority for the game's "Eye". It smoothly follows 
//! a target (the Player) using advanced cinematic features like lookahead 
//! bias, platform snapping, and dynamic trauma-based shake effects.

use crate::math::Vector2D;
use crate::ecs::component::Direction;

/// Holds the state and logic for the cinematic viewport.
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
    pub locked_y_position: f32,
    pub _vertical_snap_threshold: f32,
    pub vertical_tightness: f32,
    pub camera_falling_tightness: f32,
    pub camera_falling_velocity_threshold: f32,
    pub entity_max_fall_speed: f32,
    pub lookahead_distance: f32,
    pub current_lookahead_offset: f32,
    pub smoothing_speed: f32,
    pub shake_offset: Vector2D,
}

impl Camera {
    #[allow(clippy::too_many_arguments)]
    pub fn new(x: f32, y: f32, tightness: f32, virtual_width: f32, virtual_height: f32, map_width: f32, map_height: f32, slow_zone: f32, fast_zone: f32, vertical_snap_threshold: f32, vertical_tightness: f32, camera_falling_tightness: f32, camera_falling_velocity_threshold: f32, entity_max_fall_speed: f32, lookahead_distance: f32, smoothing_speed: f32) -> Self {
        Self {
            position: Vector2D::new(x, y), velocity: Vector2D::default(), tightness, virtual_width, virtual_height,
            map_width, map_height, slow_zone, fast_zone, locked_y_position: y, _vertical_snap_threshold: vertical_snap_threshold,
            vertical_tightness, camera_falling_tightness, camera_falling_velocity_threshold, entity_max_fall_speed,
            lookahead_distance, current_lookahead_offset: 0.0, smoothing_speed, shake_offset: Vector2D::default(),
        }
    }

    pub fn view_rect(&self) -> sdl3::rect::Rect {
        sdl3::rect::Rect::new(self.position.x as i32, self.position.y as i32, self.virtual_width as u32, self.virtual_height as u32)
    }

    /// Smoothly interpolates the camera position to track a target with cinematic logic.
    pub fn update(&mut self, target: Vector2D, is_grounded: bool, player_vel_y: f32, player_direction: Direction) {
        let slow_zone_x = self.virtual_width * self.slow_zone;
        let fast_zone_x = self.virtual_width * self.fast_zone;
        let target_lookahead_offset = if player_direction == Direction::Right { self.lookahead_distance } else { -self.lookahead_distance };
        self.current_lookahead_offset += (target_lookahead_offset - self.current_lookahead_offset) * self.smoothing_speed;
        let base_x = self.position.x - self.shake_offset.x;
        let base_y = self.position.y - self.shake_offset.y;
        let camera_center_x = base_x + self.virtual_width / 2.0;
        let delta_x = (target.x + self.current_lookahead_offset) - camera_center_x;
        let mut move_x = 0.0;
        if delta_x.abs() > slow_zone_x {
            let speed_factor = if delta_x.abs() > fast_zone_x { 1.0 } else { ((delta_x.abs() - slow_zone_x) / (fast_zone_x - slow_zone_x)).powi(3) };
            move_x = delta_x * speed_factor * self.tightness;
        }
        if is_grounded { self.locked_y_position = target.y - (self.virtual_height / 2.0); }
        let vertical_tightness = if player_vel_y > self.camera_falling_velocity_threshold {
            let t = ((player_vel_y - self.camera_falling_velocity_threshold) / (self.entity_max_fall_speed - self.camera_falling_velocity_threshold)).clamp(0.0, 1.0);
            self.vertical_tightness + t * (self.camera_falling_tightness - self.vertical_tightness)
        } else { self.vertical_tightness };
        let delta_y = self.locked_y_position - base_y;
        let move_y = delta_y * vertical_tightness;
        let new_base_x = (base_x + move_x).clamp(0.0, self.map_width - self.virtual_width);
        let new_base_y = (base_y + move_y).clamp(0.0, self.map_height - self.virtual_height);
        self.position.x = new_base_x + self.shake_offset.x;
        self.position.y = new_base_y + self.shake_offset.y;
    }

    pub fn snap_to(&mut self, target: Vector2D) {
        self.position.x = (target.x - self.virtual_width / 2.0).clamp(0.0, self.map_width - self.virtual_width);
        self.position.y = (target.y - self.virtual_height / 2.0).clamp(0.0, self.map_height - self.virtual_height);
        self.velocity = Vector2D::default();
    }
}