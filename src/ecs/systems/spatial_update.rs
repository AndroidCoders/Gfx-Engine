//! # Concept: Spatial Partitioning
//!
//! This module is responsible for maintaining the Spatial Grid, a uniform grid
//! structure used to optimize collision queries and frustum culling.
//! It ensures that entities can be efficiently located by their physical position.

use crate::ecs::systems::{System, SystemContext};

/// A system that synchronizes the Spatial Grid with the current world state.
pub struct SystemSpatialUpdate;

impl System<SystemContext<'_>> for SystemSpatialUpdate {
    /// Clears and repopulates the spatial grid with active entity collision boxes.
    fn update(&mut self, world: &mut crate::ecs::world::World, _context: &mut SystemContext<'_>) {
        // 1. Clear the grid from the previous frame's data.
        world.spatial_grid.clear();

        // 2. Collect all entities that have both physical presence (Collision) and Position.
        let entities: Vec<_> = world.collisions.keys().copied().collect();

        for entity in entities {
            // 3. Skip dormant entities to save CPU cycles; they are excluded from current partitioning.
            if world.is_dormant(entity) { continue; }

            if let (Some(pos), Some(collision)) = (world.positions.get(&entity), world.collisions.get_mut(&entity)) {
                // 4. Synchronize the abstract collision rectangle with the authoritative world position.
                collision.rect.set_x(pos.0.x as i32);
                collision.rect.set_y(pos.0.y as i32);

                // 5. Re-file the entity into the grid cells overlapped by its updated bounding box.
                world.spatial_grid.insert(entity, collision.rect);
            }
        }
    }
}