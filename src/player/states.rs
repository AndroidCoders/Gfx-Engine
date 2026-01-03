//! # Concept: Player Behavior (HSM)
//!
//! This module defines the logical states for the player character. 
//! it implements the Hierarchical State Machine (HSM) that determines 
//! how the player responds to physics and input facts.

use crate::state_machine::State;
use crate::ecs::world::{World, Entity};
use crate::ecs::systems::SystemContext;
use crate::input::InputAction;
use crate::audio::{AudioEvent, PlaySoundParams};

/// # Concept: Idle State
/// Handles the stationary behavior of the player on solid ground.
pub struct IdleState;

impl State for IdleState {
    fn enter(&mut self) {}
    fn exit(&mut self) {}

    fn update_with_context(&mut self, world: &mut World, _context: &mut SystemContext, entity: Entity) {
        if let Some(anim) = world.animations.get_mut(&entity) {
            let dir = world.directions.get(&entity).map(|d| d.direction).unwrap_or(crate::ecs::component::Direction::Right);
            match dir {
                crate::ecs::component::Direction::Left => anim.controller.set_animation("idle_left"),
                crate::ecs::component::Direction::Right => anim.controller.set_animation("idle_right"),
            }
        }
    }

    fn transition_with_context(&mut self, world: &mut World, context: &mut SystemContext, entity: Entity) -> Option<Box<dyn State>> {
        let input_state = context.input_state;
        let physics_config = &context.config.physics;

        if let Some(vel) = world.velocities.get(&entity) {
            if input_state.is_action_just_pressed(InputAction::Jump) && world.is_grounded(entity) {
                if let Some(vel_mut) = world.velocities.get_mut(&entity) { vel_mut.0.y = physics_config.jump_strength; }
                if let Some(sound_name) = context.game_config.sound_events.get("player_jump") {
                    let _ = context.audio_sender.send(AudioEvent::PlaySound(sound_name.clone(), PlaySoundParams::default()));
                }
                return Some(Box::new(JumpingState));
            }

            if !world.is_grounded(entity) {
                if vel.0.y < 0.0 { return Some(Box::new(JumpingState)); } 
                else if vel.0.y > 100.0 { return Some(Box::new(FallingState)); }
            }

            // Transition to Walking if moving OR if intending to move (pushing wall).
            if vel.0.x.abs() > physics_config.velocity_threshold 
                || input_state.is_action_pressed(InputAction::MoveLeft) 
                || input_state.is_action_pressed(InputAction::MoveRight) { 
                    return Some(Box::new(WalkingState)); 
            }
        }
        None
    }
    fn get_name(&self) -> &str { "IdleState" }
}

/// # Concept: Walking State
/// Handles horizontal locomotion and orientation updates.
pub struct WalkingState;

impl State for WalkingState {
    fn enter(&mut self) {}
    fn exit(&mut self) {}

    fn update_with_context(&mut self, world: &mut World, _context: &mut SystemContext, entity: Entity) {
        if let Some(vel) = world.velocities.get(&entity) {
            let mut direction = None;
            // Prioritize input direction for orientation if velocity is blocked.
            // This prevents the sprite from flipping wildly if velocity jitter happens near 0.
            // Note: We don't have direct access to Input here easily without refactoring function signature
            // or adding MovementIntention component lookup.
            // However, SystemMovement sets velocity based on input, so vel sign is usually correct unless 0.
            
            // Fallback: If vel is 0, use MovementIntention if available.
            if vel.0.x.abs() > 0.1 {
                if vel.0.x < 0.0 { direction = Some(crate::ecs::component::Direction::Left); } 
                else { direction = Some(crate::ecs::component::Direction::Right); }
            } else if let Some(intent) = world.movement_intentions.get(&entity) {
                 if intent.x < -0.1 { direction = Some(crate::ecs::component::Direction::Left); }
                 else if intent.x > 0.1 { direction = Some(crate::ecs::component::Direction::Right); }
            }

            if let Some(dir) = direction {
                world.add_direction(entity, crate::ecs::component::Directional { direction: dir });
                if let Some(anim) = world.animations.get_mut(&entity) {
                    match dir {
                        crate::ecs::component::Direction::Left => anim.controller.set_animation("walk_left"),
                        crate::ecs::component::Direction::Right => anim.controller.set_animation("walk_right"),
                    }
                }
            }
        }
    }

    fn transition_with_context(&mut self, world: &mut World, context: &mut SystemContext, entity: Entity) -> Option<Box<dyn State>> {
        let input_state = context.input_state;
        let physics_config = &context.config.physics;

        if let Some(vel) = world.velocities.get(&entity) {
            if input_state.is_action_just_pressed(InputAction::Jump) && world.is_grounded(entity) {
                if let Some(vel_mut) = world.velocities.get_mut(&entity) { vel_mut.0.y = physics_config.jump_strength; }
                if let Some(sound_name) = context.game_config.sound_events.get("player_jump") {
                    let _ = context.audio_sender.send(AudioEvent::PlaySound(sound_name.clone(), PlaySoundParams::default()));
                }
                return Some(Box::new(JumpingState));
            }
            
            if !world.is_grounded(entity) {
                if vel.0.y < 0.0 { return Some(Box::new(JumpingState)); } 
                else if vel.0.y > 100.0 { return Some(Box::new(FallingState)); }
            }

            // Only transition to Idle if stopped AND not trying to move.
            if vel.0.x.abs() < physics_config.velocity_threshold 
                && !input_state.is_action_pressed(InputAction::MoveLeft) 
                && !input_state.is_action_pressed(InputAction::MoveRight) { 
                    return Some(Box::new(IdleState)); 
            }
        }
        None
    }
    fn get_name(&self) -> &str { "WalkingState" }
}

/// # Concept: Jumping State
/// Handles upward momentum and variable jump height (hold-to-jump-higher).
pub struct JumpingState;

impl State for JumpingState {
    fn enter(&mut self) {}
    fn exit(&mut self) {}

    fn update_with_context(&mut self, world: &mut World, context: &mut SystemContext, entity: Entity) {
        let input_state = context.input_state;
        let physics_config = &context.config.physics;

        if let Some(vel) = world.velocities.get_mut(&entity)
            && input_state.is_action_pressed(InputAction::Jump) && vel.0.y < 0.0 {
                vel.0.y -= physics_config.jump_hold_force * context.delta_time;
            }
    }

    fn transition_with_context(&mut self, world: &mut World, _context: &mut SystemContext, entity: Entity) -> Option<Box<dyn State>> {
        if let Some(vel) = world.velocities.get(&entity) && vel.0.y >= 0.0 { return Some(Box::new(FallingState)); }
        None
    }
    fn get_name(&self) -> &str { "JumpingState" }
}

/// # Concept: Falling State
/// Handles downward momentum and landing detection.
pub struct FallingState;

impl State for FallingState {
    fn enter(&mut self) {}
    fn exit(&mut self) {}
    fn update_with_context(&mut self, _world: &mut World, _context: &mut SystemContext, _entity: Entity) {}

    fn transition_with_context(&mut self, world: &mut World, _context: &mut SystemContext, entity: Entity) -> Option<Box<dyn State>> {
        if world.is_grounded(entity) { return Some(Box::new(IdleState)); }
        None
    }
    fn get_name(&self) -> &str { "FallingState" }
}

/// # Concept: Dying State
/// Manages the non-interactive death cinematic sequence.
pub struct DyingState { pub timer: f32 }

impl State for DyingState {
    fn enter(&mut self) {}
    fn exit(&mut self) {}
    fn update_with_context(&mut self, _world: &mut World, context: &mut SystemContext, _entity: Entity) {
        self.timer -= context.delta_time;
    }

    fn transition_with_context(&mut self, _world: &mut World, _context: &mut SystemContext, _entity: Entity) -> Option<Box<dyn State>> {
        if self.timer <= 0.0 { Some(Box::new(DeadState)) } else { None }
    }
    fn get_name(&self) -> &str { "DyingState" }
}

/// # Concept: Dead State
/// A terminal state representing complete loss of interactivity.
pub struct DeadState;
impl State for DeadState {
    fn enter(&mut self) {}
    fn exit(&mut self) {}
    fn update_with_context(&mut self, _world: &mut World, _context: &mut SystemContext, _entity: Entity) {}
    fn transition_with_context(&mut self, _world: &mut World, _context: &mut SystemContext, _entity: Entity) -> Option<Box<dyn State>> { None }
    fn get_name(&self) -> &str { "DeadState" }
}
