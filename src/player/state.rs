// src/player/state.rs

//! Defines the player's specific states and the logic for each state.

use crate::config::Config;
use crate::input::{InputState, PlayerAction};
use crate::level::Level;
use crate::player::{Player, PlayerDirection};
use sdl3::rect::Rect;

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
) {
    // Handle horizontal movement consistently across states
    handle_horizontal_movement(player, input_state, config);

    // Set player direction based on velocity, fixing the "sticky direction" bug.
    if player.velocity.x > 0.1 {
        player.direction = PlayerDirection::Right;
    } else if player.velocity.x < -0.1 {
        player.direction = PlayerDirection::Left;
    } else if player.is_on_ground {
        player.direction = PlayerDirection::Front;
    }

    // Determine the next state based on the current state and input.
    let next_state = match player.state {
        PlayerState::Idle => handle_idle_state(player, input_state, config),
        PlayerState::Walking => handle_walking_state(player, input_state, config),
        PlayerState::Jumping => handle_jumping_state(player, input_state, config),
        PlayerState::Falling => handle_falling_state(player, input_state, config),
    };
    player.state = next_state;

    // Update animation based on the new state
    match player.state {
        PlayerState::Idle => {
            player.animation_controller.set_animation("idle");
        }
        PlayerState::Walking => match player.direction {
            PlayerDirection::Left => player.animation_controller.set_animation("walk_left"),
            PlayerDirection::Right => player.animation_controller.set_animation("walk_right"),
            PlayerDirection::Front => player.animation_controller.set_animation("idle"),
        },
        PlayerState::Jumping | PlayerState::Falling => {
            // While airborne, use a static directional pose.
            match player.direction {
                PlayerDirection::Left => player.animation_controller.set_animation("jump_left"),
                PlayerDirection::Right => player.animation_controller.set_animation("jump_right"),
                PlayerDirection::Front => player.animation_controller.set_animation("idle"), // Fallback for vertical jumps
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
            player.ground_friction
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

fn handle_idle_state(player: &mut Player, input_state: &InputState, config: &Config) -> PlayerState {
    // Transitions
    if input_state.is_action_just_pressed(PlayerAction::Jump) {
        player.velocity.y = config.physics.jump_strength;
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
) -> PlayerState {
    // Transitions
    if input_state.is_action_just_pressed(PlayerAction::Jump) {
        player.velocity.y = config.physics.jump_strength;
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

    // Reset on_ground flag; it will be set by vertical collision if it occurs.
    player.is_on_ground = false;

    // --- Horizontal Collision ---
    player.position.x += player.velocity.x;
    let player_rect_h = Rect::new(
        player.position.x as i32,
        player.position.y as i32,
        player.width,
        player.height,
    );

    for object in &level.objects {
        let object_rect = Rect::new(object.x, object.y, object.width, object.height);
        if player_rect_h.has_intersection(object_rect) {
            if player.velocity.x > 0.0 {
                // Moving right
                player.position.x = (object.x - player.width as i32) as f32;
            } else if player.velocity.x < 0.0 {
                // Moving left
                player.position.x = (object.x + object.width as i32) as f32;
            }
            player.velocity.x = 0.0;
        }
    }

    // --- Vertical Collision ---
    player.position.y += player.velocity.y;
    let player_rect_v = Rect::new(
        player.position.x as i32,
        player.position.y as i32,
        player.width,
        player.height,
    );

    for object in &level.objects {
        let object_rect = Rect::new(object.x, object.y, object.width, object.height);
        if player_rect_v.has_intersection(object_rect) {
            if player.velocity.y > 0.0 {
                // Moving down (landing)
                player.position.y = (object.y - player.height as i32) as f32;
                player.is_on_ground = true;
                player.jump_time = 0;
                player.ground_friction = object.friction.unwrap_or(config.physics.friction);
                // Transition back to Idle or Walking
                if player.state == PlayerState::Falling || player.state == PlayerState::Jumping {
                    player.state = PlayerState::Idle;
                }
            } else if player.velocity.y < 0.0 {
                // Moving up (hitting ceiling)
                player.position.y = (object.y + object.height as i32) as f32;
            }
            player.velocity.y = 0.0;
        }
    }
}