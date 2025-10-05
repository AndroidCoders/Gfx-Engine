// src/player/mod.rs

//! Defines the player character's state and behavior.

pub mod state;

use crate::animation::AnimationController;
use crate::config::PlayerConfig;
use crate::math::Vector2D;
use state::PlayerState;

pub enum PlayerDirection {
    Left,
    Right,
}

pub struct Player {
    pub position: Vector2D,
    pub velocity: Vector2D,
    pub width: u32, // Collision width
    pub height: u32, // Collision height
    pub draw_width: u32,
    pub draw_height: u32,
    pub horizontal_draw_offset: i32,
    pub vertical_draw_offset: i32,
    pub direction: PlayerDirection,
    pub is_on_ground: bool,
    pub jump_time: u32,
    pub ground_friction: f32,
    pub state: PlayerState,
    pub animation_controller: AnimationController,
}

impl Player {
    pub fn new(config: &PlayerConfig) -> Self {
        Self {
            position: Vector2D::new(config.start_x, config.start_y),
            velocity: Vector2D::default(),
            width: config.width,
            height: config.height,
            draw_width: config.draw_width,
            draw_height: config.draw_height,
            horizontal_draw_offset: config.horizontal_draw_offset,
            vertical_draw_offset: config.vertical_draw_offset,
            direction: PlayerDirection::Right,
            is_on_ground: false,
            jump_time: 0,
            // TODO: Initialize this from config.physics.friction (air friction)
            ground_friction: 0.9,
            state: PlayerState::Idle,
            animation_controller: AnimationController::new(),
        }
    }
}
