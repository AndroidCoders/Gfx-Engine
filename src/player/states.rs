// src/player/states.rs

//! Defines the player's specific states and the logic for each state.

use crate::state_machine::State;
use crate::ecs::world::{World, Entity};
use crate::ecs::systems::SystemContext;
use crate::input::PlayerAction;
use crate::audio::AudioEvent;

pub struct IdleState;

impl State for IdleState {
    fn enter(&mut self) {
        // println!("Entering IdleState");
    }

    fn exit(&mut self) {
        // println!("Exiting IdleState");
    }

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

            // Apply friction if not actively moving horizontally
            if !is_moving_horizontally {
                vel.0.x *= physics_config.friction;
                if vel.0.x.abs() < 0.1 { // Stop if velocity is very small
                    vel.0.x = 0.0;
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

    fn transition_with_context(&mut self, world: &mut World, context: &mut SystemContext, entity: Entity) -> Option<Box<dyn State>> {
        let input_state = context.input_state;
        let physics_config = &context.config.physics;

        if let Some(vel) = world.velocities.get(&entity) {
            if input_state.is_action_just_pressed(PlayerAction::Jump) && world.is_grounded(entity) {
                if let Some(vel_mut) = world.velocities.get_mut(&entity) {
                    vel_mut.0.y = physics_config.jump_strength;
                }
                let _ = context.audio_sender.send(AudioEvent::PlayerJumped);
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

    fn get_name(&self) -> &str {
        "IdleState"
    }
}
pub struct WalkingState;

impl State for WalkingState {
    fn enter(&mut self) {
        // println!("Entering WalkingState");
    }

    fn exit(&mut self) {
        // println!("Exiting WalkingState");
    }

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
                vel.0.x *= physics_config.friction;
                if vel.0.x.abs() < 0.1 { // Stop if velocity is very small
                    vel.0.x = 0.0;
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

    fn transition_with_context(&mut self, world: &mut World, context: &mut SystemContext, entity: Entity) -> Option<Box<dyn State>> {
        let input_state = context.input_state;
        let physics_config = &context.config.physics;

        if let Some(vel) = world.velocities.get(&entity) {
            if input_state.is_action_just_pressed(PlayerAction::Jump) && world.is_grounded(entity) {
                if let Some(vel_mut) = world.velocities.get_mut(&entity) {
                    vel_mut.0.y = physics_config.jump_strength;
                }
                let _ = context.audio_sender.send(AudioEvent::PlayerJumped);
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

    fn get_name(&self) -> &str {
        "WalkingState"
    }
}
pub struct JumpingState;

impl State for JumpingState {
    fn enter(&mut self) {
        // println!("Entering JumpingState");
    }

    fn exit(&mut self) {
        // println!("Exiting JumpingState");
    }

    fn update_with_context(&mut self, world: &mut World, context: &mut SystemContext, entity: Entity) {
        let input_state = context.input_state;
        let physics_config = &context.config.physics;

        if let Some(vel) = world.velocities.get_mut(&entity) {
            // Apply jump hold force
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

    fn transition_with_context(&mut self, world: &mut World, _context: &mut SystemContext, entity: Entity) -> Option<Box<dyn State>> {
        if let Some(vel) = world.velocities.get(&entity) {
            if vel.0.y >= 0.0 {
                return Some(Box::new(FallingState));
            }
        }
        None
    }

    fn get_name(&self) -> &str {
        "JumpingState"
    }
}
pub struct FallingState;

impl State for FallingState {
    fn enter(&mut self) {
        // println!("Entering FallingState");
    }

    fn exit(&mut self) {
        // println!("Exiting FallingState");
    }

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

    fn transition_with_context(&mut self, world: &mut World, _context: &mut SystemContext, entity: Entity) -> Option<Box<dyn State>> {
        if world.is_grounded(entity) {
            return Some(Box::new(IdleState));
        }
        None
    }

    fn get_name(&self) -> &str {
        "FallingState"
    }
}