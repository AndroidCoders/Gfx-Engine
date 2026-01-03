//! # Synchronization: Animation
//!
//! This module acts as the "Eye" of the engine. It implements the rules that 
//! map an entity's physical state (velocity, direction, grounded status) 
//! to a specific sprite animation clip, decoupling gameplay logic from visuals.

use crate::ecs::systems::{System, SystemContext};
use crate::ecs::component::Direction;

/// A system that resolves the correct animation name for entities based on their state.
pub struct SystemAnimationSynchronization;

impl System<SystemContext<'_>> for SystemAnimationSynchronization {
    /// Evaluates entity state and updates animation controllers with the resolved clip names.
    fn update(&mut self, world: &mut crate::ecs::world::World, _context: &mut SystemContext<'_>) {
        let entities: Vec<_> = world.animations.keys().copied().collect();

        for entity in entities {
            // Skip entities outside the active simulation range.
            if world.is_dormant(entity) { continue; }

            let mut next_anim = None;

            // --- Rule Sets ---

            // Rule Set 1: Resolve animations for the Player character.
            if world.player_tags.contains_key(&entity) {
                next_anim = self.resolve_player_animation(world, entity);
            } 
            // Rule Set 2: Resolve animations for Patrolling enemies.
            else if let Some(patrol) = world.patrols.get(&entity) {
                next_anim = self.resolve_patrol_animation(world, entity, &patrol.anim_prefix);
            }

            // --- Application ---
            if let Some(anim_name) = next_anim
                && let Some(animation) = world.animations.get_mut(&entity) {
                    // Only update the controller if the animation has changed to avoid resetting frames.
                    if animation.controller.current_animation_name() != Some(&anim_name)
                        && animation.controller.has_animation(&anim_name) {
                        animation.controller.set_animation(&anim_name);
                    }
            }
        }
    }
}

impl SystemAnimationSynchronization {
    /// Maps player physical state (grounded, velocity, direction) to animation clip names.
    fn resolve_player_animation(&self, world: &crate::ecs::world::World, entity: crate::ecs::world::Entity) -> Option<String> {
        // Check for priority states like 'Dying' that override standard movement visuals.
        if let Some(state_comp) = world.state_components.get(&entity) {
            let state_name = state_comp.state_machine.current_state.as_ref().map(|s| s.get_name()).unwrap_or("");
            if state_name == "DyingState" || state_name == "DeadState" {
                return None; 
            }
        }

        let is_grounded = world.is_grounded(entity);
        let vel = world.velocities.get(&entity).map(|v| v.0).unwrap_or_default();
        let intent = world.movement_intentions.get(&entity).map(|i| i.x).unwrap_or(0.0);
        let dir = world.directions.get(&entity).map(|d| d.direction).unwrap_or(Direction::Right);

        // Resolve based on air/ground status and horizontal speed.
        let anim = if !is_grounded {
            match dir {
                Direction::Left => "jump_left",
                Direction::Right => "jump_right",
            }
        } else if vel.x.abs() > 0.1 || intent.abs() > 0.1 {
            match dir {
                Direction::Left => "walk_left",
                Direction::Right => "walk_right",
            }
        } else {
            match dir {
                Direction::Left => "idle_left",
                Direction::Right => "idle_right",
            }
        };

        Some(anim.to_string())
    }

    /// Maps patrol velocity and direction to prefixed animation clip names.
    fn resolve_patrol_animation(&self, world: &crate::ecs::world::World, entity: crate::ecs::world::Entity, prefix: &str) -> Option<String> {
        let vel = world.velocities.get(&entity).map(|v| v.0).unwrap_or_default();
        
        let suffix = if vel.x.abs() > 0.1 {
            if vel.x > 0.0 { "walk_right" } else { "walk_left" }
        } else {
            // Fallback to intended direction if the entity is currently stationary.
            if let Some(patrol) = world.patrols.get(&entity) {
                if patrol.direction > 0.0 { "walk_right" } else { "walk_left" }
            } else {
                "walk_right"
            }
        };

        Some(format!("{}_{}", prefix, suffix))
    }
}