//! # Concept: Spatial Partitioning
//!
//! This module is responsible for maintaining the Spatial Grid, a uniform grid
//! structure used to optimize collision queries and frustum culling.
//! It ensures that entities can be efficiently located by their physical position.

use crate::ecs::systems::{System, SystemContext};

/// A system that synchronizes the Spatial Grid with the current world state.
pub struct SystemSpatialUpdate;

impl System<SystemContext<'_>> for SystemSpatialUpdate {
    /// Re-indexes all collidable entities into the spatial grid based on their current bounds.
    ///
    /// ⚠️ **Hotpath**: Called 120x per second. Iterates over all physical entities.
    fn update(&mut self, world: &mut crate::ecs::world::World, _context: &mut SystemContext<'_>) {
        // 1. Clear the previous frame's spatial index.
        world.spatial_grid.clear();

        // 2. Iterate over all entities with a collision component.
        for (entity, collision) in &world.collisions {
            // 3. Insert them into the grid based on their current bounding box.
            world.spatial_grid.insert(*entity, collision.rect);
        }
    }
}