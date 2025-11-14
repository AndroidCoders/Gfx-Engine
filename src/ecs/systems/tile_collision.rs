//! This system is responsible for handling collisions between entities and the tile-based game world.

use crate::ecs::component::Grounded;
use crate::ecs::systems::{System, SystemContext};
use crate::ecs::world::World;
use crate::physics;

/// The system that resolves collisions between entities and solid tiles in the level.
pub struct TileCollisionSystem;
impl System<SystemContext<'_>> for TileCollisionSystem {
    /// Updates the system, resolving collisions for all entities that have
    /// `Position`, `Velocity`, and `Collision` components.
    ///
    /// This system first clears all `Grounded` tags from entities. Then, for each
    /// relevant entity, it updates its collision rectangle and calls the physics
    /// functions to resolve vertical and horizontal collisions with the level's
    /// solid tiles. If a vertical collision results in the entity being grounded,
    /// a `Grounded` tag is added.
    fn update(&mut self, world: &mut World, context: &mut SystemContext) {
        world.grounded_tags.clear();
        let mut entities_to_ground = Vec::new();

        for (entity, pos) in world.positions.iter_mut() {
            if let (Some(vel), Some(collision)) = (world.velocities.get_mut(entity), world.collisions.get_mut(entity)) {
                // Update the collision rectangle's position to match the entity's current position.
                collision.rect.set_x(pos.0.x as i32);
                collision.rect.set_y(pos.0.y as i32);

                // Resolve vertical and horizontal collisions with the tilemap.
                let grounded = physics::resolve_vertical_collisions(pos, vel, collision.rect, context);
                physics::resolve_horizontal_collisions(pos, vel, collision.rect, context);

                if grounded {
                    entities_to_ground.push(*entity);
                }
            }
        }

        // Add Grounded tags to entities that are now on a solid surface.
        for entity in entities_to_ground {
            world.add_grounded(entity, Grounded);
        }
    }
}
