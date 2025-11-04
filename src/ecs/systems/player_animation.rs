//! This system is responsible for updating the player's animation based on their state and direction.

use crate::ecs::component::Direction;
use crate::ecs::systems::{System, SystemContext};
use crate::ecs::world::World;

/// The `PlayerAnimationSystem` updates the player's animation based on their state and direction.
pub struct PlayerAnimationSystem;
impl System<SystemContext<'_>> for PlayerAnimationSystem {
    /// Updates the system, setting the player's animation based on their state and direction.
    fn update(&mut self, world: &mut World, _context: &mut SystemContext) {
        for (entity, _) in &world.player_tags {
            // Update the player's direction based on their velocity.
            if let Some(vel) = world.velocities.get(entity) {
                if vel.0.x < 0.0 {
                    if let Some(dir) = world.directions.get_mut(entity) {
                        dir.direction = Direction::Left;
                    }
                } else if vel.0.x > 0.0 {
                    if let Some(dir) = world.directions.get_mut(entity) {
                        dir.direction = Direction::Right;
                    }
                }
            }

            // Update the player's animation based on their state and direction.
            if let Some(animation) = world.animations.get_mut(entity) {
                if let Some(state_component) = world.state_components.get(entity) {
                    let current_state_name = state_component.state_machine.current_state.as_ref().map_or("IdleState", |s| s.get_name());

                    let direction_str = if let Some(dir) = world.directions.get(entity) {
                        if dir.direction == Direction::Left { "left" } else { "right" }
                    } else {
                        "right"
                    };

                    let animation_name = match current_state_name {
                        "IdleState" => format!("idle_{}", direction_str),
                        "WalkingState" => format!("walk_{}", direction_str),
                        "JumpingState" => format!("jump_{}", direction_str),
                        "FallingState" => format!("fall_{}", direction_str),
                        _ => format!("idle_{}", direction_str), // Fallback
                    };
                    
                    if animation.controller.has_animation(&animation_name) {
                        animation.controller.set_animation(&animation_name);
                    } else {
                        animation.controller.set_animation(&format!("idle_{}", direction_str));
                    }
                }
            }
        }
    }
}
