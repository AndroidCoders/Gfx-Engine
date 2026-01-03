//! # Concept: Simulation Culling (Dormancy)
//!
//! This module implements the "Macro-Culling" optimization. It tags entities 
//! outside the camera's active range as 'Dormant', excluding them from 
//! expensive simulation systems like Physics, AI, and Animation.

use crate::ecs::systems::{System, SystemContext};
use crate::ecs::component::DormantTag;

/// A system that manages entity dormancy based on distance from the camera.
pub struct SystemDormancy;

impl System<SystemContext<'_>> for SystemDormancy {
    /// Updates the dormancy state of entities based on camera distance.
    ///
    /// ⚠️ **Hotpath**: Called 120x per second.
    fn update(&mut self, world: &mut crate::ecs::world::World, context: &mut SystemContext<'_>) {
        // 1. Define the active simulation zone (e.g., 2 screens wide).
        let camera_x = context.camera.position.x;
        let viewport_width = context.config.window.virtual_width as f32;
        let active_margin = viewport_width * 1.5;
        
        let min_x = camera_x - active_margin;
        let max_x = camera_x + viewport_width + active_margin;

        // 2. Iterate over all entities with positions.
        // We collect to avoid borrowing conflicts, though iterating query is better if possible.
        let entities: Vec<_> = world.positions.keys().copied().collect();

        for entity in entities {
            // We generally don't cull the player.
            if world.player_tags.contains_key(&entity) { continue; }

            if let Some(pos) = world.positions.get(&entity) {
                let x = pos.0.x;
                
                // 3. Check bounds and toggle DormantTag.
                if x < min_x || x > max_x {
                    if !world.dormant_tags.contains_key(&entity) {
                        world.add_dormant_tag(entity, crate::ecs::component::DormantTag);
                    }
                } else {
                    if world.dormant_tags.contains_tag(&entity) {
                        world.dormant_tags.remove(&entity);
                    }
                }
            }
        }
    }
}