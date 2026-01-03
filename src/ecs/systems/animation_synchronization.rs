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
    /// Synchronizes entity animation state with its physical and logical state.
    ///
    /// ⚠️ **Hotpath**: Called 120x per second.
    fn update(&mut self, world: &mut crate::ecs::world::World, _context: &mut SystemContext<'_>) {
        // 1. Synchronize Player Animation
        let player_entities: Vec<_> = world.player_tags.keys().copied().collect();
        for entity in player_entities {
            // Priority 1: High-priority states (Damage/Death) override everything.
            if let Some(state_comp) = world.state_components.get(&entity) {
                let state_name = state_comp.state_machine.current_state.as_ref().map(|s| s.get_name()).unwrap_or("");
                if state_name == "DyingState" || state_name == "DeadState" {
                    // Animation is handled by the state machine entry/exit logic for these states.
                    continue; 
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