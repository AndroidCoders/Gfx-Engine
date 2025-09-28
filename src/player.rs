// src/player.rs

//! Defines the player character's state and behavior.

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
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: Vector2D::new(x, y),
            velocity: Vector2D::default(),
            width: 64,
            height: 128,
            direction: PlayerDirection::Front,
            is_on_ground: false,
        }
    }
}