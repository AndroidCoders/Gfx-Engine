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
    /// Calculates entity visibility and toggles the DormantTag to optimize simulation.
    fn update(&mut self, world: &mut crate::ecs::world::World, context: &mut SystemContext<'_>) {
        // 1. Define the 'Active Zone' dimensions centered on the current camera position.
        let active_width = 3840.0;
        let active_height = 2160.0;

        let cam_center_x = context.camera.position.x + (context.camera.virtual_width / 2.0);
        let cam_center_y = context.camera.position.y + (context.camera.virtual_height / 2.0);

        let min_x = cam_center_x - (active_width / 2.0);
        let max_x = cam_center_x + (active_width / 2.0);
        let min_y = cam_center_y - (active_height / 2.0);
        let max_y = cam_center_y + (active_height / 2.0);

        // 2. Iterate over all entities with positions to evaluate their dormancy status.
        let entities: Vec<_> = world.positions.keys().copied().collect();

        for entity in entities {
            // 3. Ensure the player entity is always active regardless of camera distance.
            if world.player_tags.contains_key(&entity) {
                world.remove_dormant_tag(entity);
                continue;
            }

            if let Some(pos) = world.positions.get(&entity) {
                let x = pos.0.x;
                let y = pos.0.y;

                // 4. Determine if the entity's position falls within the Active Zone.
                let is_inside = x >= min_x && x <= max_x && y >= min_y && y <= max_y;

                if is_inside {
                    // 5. Activate entities that have entered the zone.
                    world.remove_dormant_tag(entity);
                } else {
                    // 6. Hibernate entities that have left the zone to save CPU resources.
                    world.add_dormant_tag(entity, DormantTag);
                }
            }
        }
    }
}