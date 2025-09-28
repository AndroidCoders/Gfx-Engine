// src/player.rs

//! Defines the player character's state and behavior.

use crate::config::PlayerConfig;
use crate::math::Vector2D;

pub enum PlayerDirection {
    Front,
    Left,
    Right,
}

pub struct Player {
    pub position: Vector2D,
    pub velocity: Vector2D,
    pub width: u32,
    pub height: u32,
    pub direction: PlayerDirection,
    pub is_on_ground: bool,
    pub jump_time: u32,
}

impl Player {
    pub fn new(config: &PlayerConfig) -> Self {
        Self {
            position: Vector2D::new(config.start_x, config.start_y),
            velocity: Vector2D::default(),
            width: config.width,
            height: config.height,
            direction: PlayerDirection::Front,
            is_on_ground: false,
            jump_time: 0,
        }
    }
}