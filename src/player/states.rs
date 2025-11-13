// src/player/states.rs

//! This module defines the specific states for the player character.
//!
//! Each state struct (e.g., `IdleState`, `WalkingState`) implements the `State` trait
//! and contains the logic for the player's behavior and transitions between states.

use crate::state_machine::State;
use crate::ecs::world::{World, Entity};
use crate::ecs::systems::SystemContext;
use crate::input::PlayerAction;
use crate::audio::AudioEvent;

/// The state for when the player is standing still on the ground.
pub struct IdleState;

impl State for IdleState {
    /// Called when the player enters the `IdleState`.
    fn enter(&mut self) {
        // println!("Entering IdleState"); // Debug print
    }

    /// Called when the player exits the `IdleState`.
    fn exit(&mut self) {
        // println!("Exiting IdleState"); // Debug print
    }

    /// Updates the player's horizontal velocity based on input, applying deceleration if no input.
    fn update_with_context(&mut self, world: &mut World, context: &mut SystemContext, entity: Entity) {
        let input_state = context.input_state;
        let physics_config = &context.config.physics;

        if let Some(vel) = world.velocities.get_mut(&entity) {
            let mut is_moving_horizontally = false;
            if input_state.is_action_active(PlayerAction::MoveLeft) {
                vel.0.x -= physics_config.acceleration;
                is_moving_horizontally = true;
            }
            if input_state.is_action_active(PlayerAction::MoveRight) {
                vel.0.x += physics_config.acceleration;
                is_moving_horizontally = true;
            }

            if !is_moving_horizontally {
                if vel.0.x > 0.0 {
                    vel.0.x = (vel.0.x - physics_config.deceleration).max(0.0);
                } else if vel.0.x < 0.0 {
                    vel.0.x = (vel.0.x + physics_config.deceleration).min(0.0);
                }
            }

            // Clamp horizontal velocity
            vel.0.x = vel.0.x.clamp(-physics_config.max_speed, physics_config.max_speed);

            #[cfg(feature = "debug-player")]
            if let Some(pos) = world.positions.get(&entity) {
                println!("IdleState: pos: {:?}, vel: {:?}", pos, vel);
            }
        }
    }

    /// Determines if the player should transition to `JumpingState`, `FallingState`, or `WalkingState`.
    fn transition_with_context(&mut self, world: &mut World, context: &mut SystemContext, entity: Entity) -> Option<Box<dyn State>> {
        let input_state = context.input_state;
        let physics_config = &context.config.physics;

        if let Some(vel) = world.velocities.get(&entity) {
            if input_state.is_action_just_pressed(PlayerAction::Jump) && world.is_grounded(entity) {
                if let Some(vel_mut) = world.velocities.get_mut(&entity) {
                    vel_mut.0.y = physics_config.jump_strength;
                }
                if let Some(sound_name) = context.game_config.sound_events.get("player_jump") {
                    let _ = context.audio_sender.send(AudioEvent::PlaySound(sound_name.clone()));
                }
                return Some(Box::new(JumpingState));
            }
            if !world.is_grounded(entity) && vel.0.y > 0.0 {
                return Some(Box::new(FallingState));
            }
            if vel.0.x.abs() > 0.1 {
                return Some(Box::new(WalkingState));
            }
        }
        None
    }

    /// Returns the name of this state for debugging purposes.
    fn get_name(&self) -> &str {
        "IdleState"
    }
}

/// The state for when the player is walking horizontally on the ground.
pub struct WalkingState;

impl State for WalkingState {
    /// Called when the player enters the `WalkingState`.
    fn enter(&mut self) {
        // println!("Entering WalkingState"); // Debug print
    }

    /// Called when the player exits the `WalkingState`.
    fn exit(&mut self) {
        // println!("Exiting WalkingState"); // Debug print
    }

    /// Updates the player's horizontal velocity based on input, applying deceleration if no input.
    fn update_with_context(&mut self, world: &mut World, context: &mut SystemContext, entity: Entity) {
        let input_state = context.input_state;
        let physics_config = &context.config.physics;

        if let Some(vel) = world.velocities.get_mut(&entity) {
            let mut is_moving_horizontally = false;
            if input_state.is_action_active(PlayerAction::MoveLeft) {
                vel.0.x -= physics_config.acceleration;
                is_moving_horizontally = true;
            }
            if input_state.is_action_active(PlayerAction::MoveRight) {
                vel.0.x += physics_config.acceleration;
                is_moving_horizontally = true;
            }

            if !is_moving_horizontally {
                if vel.0.x > 0.0 {
                    vel.0.x = (vel.0.x - physics_config.deceleration).max(0.0);
                } else if vel.0.x < 0.0 {
                    vel.0.x = (vel.0.x + physics_config.deceleration).min(0.0);
                }
            }

            // Clamp horizontal velocity
            vel.0.x = vel.0.x.clamp(-physics_config.max_speed, physics_config.max_speed);

            #[cfg(feature = "debug-player")]
            if let Some(pos) = world.positions.get(&entity) {
                println!("WalkingState: pos: {:?}, vel: {:?}", pos, vel);
            }
        }
    }

    /// Determines if the player should transition to `JumpingState`, `FallingState`, or `IdleState`.
    fn transition_with_context(&mut self, world: &mut World, context: &mut SystemContext, entity: Entity) -> Option<Box<dyn State>> {
        let input_state = context.input_state;
        let physics_config = &context.config.physics;

        if let Some(vel) = world.velocities.get(&entity) {
            if input_state.is_action_just_pressed(PlayerAction::Jump) && world.is_grounded(entity) {
                if let Some(vel_mut) = world.velocities.get_mut(&entity) {
                    vel_mut.0.y = physics_config.jump_strength;
                }
                if let Some(sound_name) = context.game_config.sound_events.get("player_jump") {
                    let _ = context.audio_sender.send(AudioEvent::PlaySound(sound_name.clone()));
                }
                return Some(Box::new(JumpingState));
            }
            if !world.is_grounded(entity) && vel.0.y > 0.0 {
                return Some(Box::new(FallingState));
            }
            if vel.0.x.abs() < 0.1 {
                return Some(Box::new(IdleState));
            }
        }
        None
    }

    /// Returns the name of this state for debugging purposes.
    fn get_name(&self) -> &str {
        "WalkingState"
    }
}

/// The state for when the player is moving upwards after a jump.
pub struct JumpingState;

impl State for JumpingState {
    /// Called when the player enters the `JumpingState`.
    fn enter(&mut self) {
        // println!("Entering JumpingState"); // Debug print
    }

    /// Called when the player exits the `JumpingState`.
    fn exit(&mut self) {
        // println!("Exiting JumpingState"); // Debug print
    }

    /// Applies jump hold force and allows for reduced horizontal air control.
    fn update_with_context(&mut self, world: &mut World, context: &mut SystemContext, entity: Entity) {
        let input_state = context.input_state;
        let physics_config = &context.config.physics;

        if let Some(vel) = world.velocities.get_mut(&entity) {
            // Apply jump hold force for variable jump height
            if input_state.is_action_active(PlayerAction::Jump)
                && vel.0.y < 0.0 { // Only apply if still moving upwards
                    vel.0.y -= physics_config.jump_hold_force;
                }

            // Allow some horizontal control while jumping
            if input_state.is_action_active(PlayerAction::MoveLeft) {
                vel.0.x -= physics_config.acceleration / 2.0; // Reduced air control
            }
            if input_state.is_action_active(PlayerAction::MoveRight) {
                vel.0.x += physics_config.acceleration / 2.0; // Reduced air control
            }

            // Clamp horizontal velocity
            vel.0.x = vel.0.x.clamp(-physics_config.max_speed, physics_config.max_speed);

            #[cfg(feature = "debug-player")]
            if let Some(pos) = world.positions.get(&entity) {
                println!("JumpingState: pos: {:?}, vel: {:?}", pos, vel);
            }
        }
    }

    /// Determines if the player should transition to `FallingState`.
    fn transition_with_context(&mut self, world: &mut World, _context: &mut SystemContext, entity: Entity) -> Option<Box<dyn State>> {
        if let Some(vel) = world.velocities.get(&entity)
            && vel.0.y >= 0.0 { // Player has reached the apex of the jump or is starting to fall
                return Some(Box::new(FallingState));
            }
        None
    }

    /// Returns the name of this state for debugging purposes.
    fn get_name(&self) -> &str {
        "JumpingState"
    }
}

/// The state for when the player is falling downwards.
pub struct FallingState;

impl State for FallingState {
    /// Called when the player enters the `FallingState`.
    fn enter(&mut self) {
        // println!("Entering FallingState"); // Debug print
    }

    /// Called when the player exits the `FallingState`.
    fn exit(&mut self) {
        // println!("Exiting FallingState"); // Debug print
    }

    /// Allows for reduced horizontal air control while falling.
    fn update_with_context(&mut self, world: &mut World, context: &mut SystemContext, entity: Entity) {
        let input_state = context.input_state;
        let physics_config = &context.config.physics;

        if let Some(vel) = world.velocities.get_mut(&entity) {
            // Allow some horizontal control while falling
            if input_state.is_action_active(PlayerAction::MoveLeft) {
                vel.0.x -= physics_config.acceleration / 2.0; // Reduced air control
            }
            if input_state.is_action_active(PlayerAction::MoveRight) {
                vel.0.x += physics_config.acceleration / 2.0; // Reduced air control
            }

            // Clamp horizontal velocity
            vel.0.x = vel.0.x.clamp(-physics_config.max_speed, physics_config.max_speed);

            #[cfg(feature = "debug-player")]
            if let Some(pos) = world.positions.get(&entity) {
                println!("FallingState: pos: {:?}, vel: {:?}", pos, vel);
            }
        }
    }

    /// Determines if the player should transition to `IdleState` (upon landing).
    fn transition_with_context(&mut self, world: &mut World, _context: &mut SystemContext, entity: Entity) -> Option<Box<dyn State>> {
        if world.is_grounded(entity) {
            return Some(Box::new(IdleState));
        }
        None
    }

    /// Returns the name of this state for debugging purposes.
    fn get_name(&self) -> &str {
        "FallingState"
    }
}

/// The state for when the player is in the process of dying.
pub struct DyingState {
    /// The remaining time in seconds for the death animation to play.
    pub timer: f32,
}

impl State for DyingState {
    /// Called when the player enters the `DyingState`.
    fn enter(&mut self) {
        // println!("Entering DyingState"); // Debug print
    }

    /// Called when the player exits the `DyingState`.
    fn exit(&mut self) {
        // println!("Exiting DyingState"); // Debug print
    }

    /// Decrements the death animation timer.
    fn update_with_context(&mut self, _world: &mut World, context: &mut SystemContext, _entity: Entity) {
        self.timer -= context.delta_time;
    }

    /// Determines if the player should transition to `IdleState` (after death animation)
    /// and be tagged for respawn.
    fn transition_with_context(&mut self, world: &mut World, _context: &mut SystemContext, entity: Entity) -> Option<Box<dyn State>> {
        if self.timer <= 0.0 {
            world.add_respawn_tag(entity, crate::ecs::component::RespawnTag);
            return Some(Box::new(IdleState));
        }
        None
    }

    /// Returns the name of this state for debugging purposes.
    fn get_name(&self) -> &str {
        "DyingState"
    }
}