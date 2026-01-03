//! # Concept: Animation Advancement
//! 
//! This module is responsible for the temporal progression of visuals.
//! it advances the internal frame counters of all active entity animations
//! based on the engine's delta time.

use crate::ecs::systems::{System, SystemContext};

/// A system that increments animation timers and frame indices.
pub struct SystemAnimationUpdate;

impl System<SystemContext<'_>> for SystemAnimationUpdate {
    /// Updates the playback position for all active animations.
    fn update(&mut self, world: &mut crate::ecs::world::World, context: &mut SystemContext<'_>) {
        // 1. Iterate over every entity currently playing an animation.
        for animation in world.animations.values_mut() {
            // 2. Advance the controller by the frame's elapsed time.
            animation.controller.update(context.delta_time);
        }
    }
}
