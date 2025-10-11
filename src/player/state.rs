// src/player/state.rs

//! Defines the player's specific states and the logic for each state.

use crate::audio::AudioEvent;
use crate::config::Config;
use crate::input::{InputState, PlayerAction};
use crate::level::Level;
use crate::player::{Player, PlayerDirection};
use std::sync::mpsc;

/// Represents the distinct states the player character can be in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerState {
    Idle,
    Walking,
    Jumping,
    Falling,
}

/// The main update function for the player's state machine.
/// This function is called from the main game loop.
pub fn update_player_state(
    player: &mut Player,
    input_state: &InputState,
    config: &Config,
    level: &Level,
    audio_event_sender: &mpsc::Sender<AudioEvent>,
) {
    // Handle horizontal movement consistently across states
    handle_horizontal_movement(player, input_state, config);

    // Set player direction based on velocity, fixing the "sticky direction" bug.
    if player.velocity.x > 0.1 {
        player.direction = PlayerDirection::Right;
    } else if player.velocity.x < -0.1 {
        player.direction = PlayerDirection::Left;
    }

    // Determine the next state based on the current state and input.
    let next_state = match player.state {
        PlayerState::Idle => handle_idle_state(player, input_state, config, audio_event_sender),
        PlayerState::Walking => handle_walking_state(player, input_state, config, audio_event_sender),
        PlayerState::Jumping => handle_jumping_state(player, input_state, config),
        PlayerState::Falling => handle_falling_state(player, input_state, config),
    };
    player.state = next_state;

    // Update animation based on the new state
    match player.state {
        PlayerState::Idle => match player.direction {
            PlayerDirection::Left => player.animation_controller.set_animation("idle_left"),
            PlayerDirection::Right => player.animation_controller.set_animation("idle_right"),
        },
        PlayerState::Walking => match player.direction {
            PlayerDirection::Left => player.animation_controller.set_animation("walk_left"),
            PlayerDirection::Right => player.animation_controller.set_animation("walk_right"),
        },
        PlayerState::Jumping | PlayerState::Falling => {
            // While airborne, use a static directional pose.
            match player.direction {
                PlayerDirection::Left => player.animation_controller.set_animation("jump_left"),
                PlayerDirection::Right => player.animation_controller.set_animation("jump_right"),
            }
        }
    }
    player.animation_controller.update();

    // Apply physics and collision detection regardless of state.
    apply_physics(player, config, level);
}

// --- Shared Logic ---

fn handle_horizontal_movement(player: &mut Player, input_state: &InputState, config: &Config) {
    let mut is_moving = false;
    if input_state.is_action_active(PlayerAction::MoveLeft) {
        player.velocity.x -= config.physics.acceleration;
        is_moving = true;
    }
    if input_state.is_action_active(PlayerAction::MoveRight) {
        player.velocity.x += config.physics.acceleration;
        is_moving = true;
    }

    // Apply friction
    if !is_moving {
        let friction = if player.is_on_ground {
            // TODO: Get ground friction from tile properties
            config.physics.friction
        } else {
            config.physics.friction // Air friction
        };
        player.velocity.x *= friction;
        if player.velocity.x.abs() < 0.1 {
            player.velocity.x = 0.0;
        }
    }

    // Clamp velocity
    player.velocity.x = player
        .velocity
        .x
        .clamp(-config.physics.max_speed, config.physics.max_speed);
}

// --- State-Specific Logic ---

fn handle_idle_state(player: &mut Player, input_state: &InputState, config: &Config, audio_event_sender: &mpsc::Sender<AudioEvent>) -> PlayerState {
    // Transitions
    if input_state.is_action_just_pressed(PlayerAction::Jump) {
        player.velocity.y = config.physics.jump_strength;
        let _ = audio_event_sender.send(AudioEvent::PlayerJumped); // Emit jump event
        return PlayerState::Jumping;
    }
    if !player.is_on_ground {
        return PlayerState::Falling;
    }
    if player.velocity.x.abs() > 0.1 {
        return PlayerState::Walking;
    }

    PlayerState::Idle
}

fn handle_walking_state(
    player: &mut Player,
    input_state: &InputState,
    config: &Config,
    audio_event_sender: &mpsc::Sender<AudioEvent>,
) -> PlayerState {
    // Transitions
    if input_state.is_action_just_pressed(PlayerAction::Jump) {
        player.velocity.y = config.physics.jump_strength;
        let _ = audio_event_sender.send(AudioEvent::PlayerJumped); // Emit jump event
        return PlayerState::Jumping;
    }
    if !player.is_on_ground {
        return PlayerState::Falling;
    }
    if player.velocity.x.abs() < 0.1 {
        return PlayerState::Idle;
    }

    PlayerState::Walking
}

fn handle_jumping_state(
    player: &mut Player,
    input_state: &InputState,
    config: &Config,
) -> PlayerState {
    // Apply jump hold force
    if input_state.is_action_active(PlayerAction::Jump)
        && player.velocity.y < 0.0
        && player.jump_time < config.physics.max_jump_time
    {
        player.velocity.y -= config.physics.jump_hold_force;
        player.jump_time += 1;
    } else {
        // End jump hold if button is released or time is up
        player.jump_time = config.physics.max_jump_time;
    }

    // Transition
    if player.velocity.y >= 0.0 {
        return PlayerState::Falling;
    }

    PlayerState::Jumping
}

fn handle_falling_state(
    _player: &mut Player,
    _input_state: &InputState,
    _config: &Config,
) -> PlayerState {
    // In this simple model, the main transition out of Falling
    // happens in `apply_physics` when `is_on_ground` becomes true.
    PlayerState::Falling
}

// --- Common Physics and Collision Logic ---

fn apply_physics(player: &mut Player, config: &Config, level: &Level) {
    // Apply gravity
    player.velocity.y += config.physics.gravity;
    player.velocity.y = player.velocity.y.min(config.physics.max_fall_speed);

    // Reset on_ground flag; it will be set by vertical collision if it occurs.
    player.is_on_ground = false;

    let scale = 2.0;
    let tile_width = level.tileset.tile_width as f32 * scale;
    let tile_height = level.tileset.tile_height as f32 * scale;

    // --- Horizontal Collision ---
    player.position.x += player.velocity.x;
    let player_rect = sdl3::rect::Rect::new(
        player.position.x as i32,
        player.position.y as i32,
        player.width,
        player.height,
    );

    let start_y = (player_rect.y() as f32 / tile_height).floor() as usize;
    let end_y = ((player_rect.y() + player_rect.height() as i32) as f32 / tile_height).ceil() as usize;

    if player.velocity.x > 0.0 { // Moving right
        let tile_x = ((player_rect.x() + player_rect.width() as i32) as f32 / tile_width).floor() as usize;
        for y in start_y..end_y {
            if let Some(row) = level.collision.tiles.get(y) {
                if let Some(&tile_id) = row.get(tile_x) {
                    if tile_id == 1 { // Solid tile
                        player.position.x = (tile_x as f32 * tile_width) - player.width as f32;
                        player.velocity.x = 0.0;
                        break;
                    }
                }
            }
        }
    } else if player.velocity.x < 0.0 { // Moving left
        let tile_x = (player_rect.x() as f32 / tile_width).floor() as usize;
        for y in start_y..end_y {
            if let Some(row) = level.collision.tiles.get(y) {
                if let Some(&tile_id) = row.get(tile_x) {
                    if tile_id == 1 { // Solid tile
                        player.position.x = (tile_x as f32 * tile_width) + tile_width;
                        player.velocity.x = 0.0;
                        break;
                    }
                }
            }
        }
    }

    // --- Vertical Collision ---
    player.position.y += player.velocity.y;
    let player_rect = sdl3::rect::Rect::new(
        player.position.x as i32,
        player.position.y as i32,
        player.width,
        player.height,
    );

    let start_x = (player_rect.x() as f32 / tile_width).floor() as usize;
    let end_x = ((player_rect.x() + player_rect.width() as i32) as f32 / tile_width).ceil() as usize;

    if player.velocity.y > 0.0 { // Moving down
        let tile_y = ((player.position.y + player.height as f32) / tile_height).floor() as usize;
        for x in start_x..end_x {
            if let Some(row) = level.collision.tiles.get(tile_y) {
                if let Some(&tile_id) = row.get(x) {
                    if tile_id == 1 { // Solid tile
                        player.position.y = (tile_y as f32 * tile_height) - player.height as f32;
                        player.velocity.y = 0.0;
                        player.is_on_ground = true;
                        player.jump_time = 0;
                        if player.state == PlayerState::Falling || player.state == PlayerState::Jumping {
                            player.state = PlayerState::Idle;
                        }
                        break;
                    }
                }
            }
        }
    } else if player.velocity.y < 0.0 { // Moving up
        let tile_y = (player_rect.y() as f32 / tile_height).floor() as usize;
        for x in start_x..end_x {
            if let Some(row) = level.collision.tiles.get(tile_y) {
                if let Some(&tile_id) = row.get(x) {
                    if tile_id == 1 { // Solid tile
                        player.position.y = (tile_y as f32 * tile_height) + tile_height;
                        player.velocity.y = 0.0;
                        break;
                    }
                }
            }
        }
    }
}