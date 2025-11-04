//! This system is responsible for handling collisions between entities and the tile-based game world.

use crate::ecs::component::Grounded;
use crate::ecs::systems::{System, SystemContext};
use crate::ecs::world::World;
use crate::physics;

/// The `TileCollisionSystem` resolves collisions between entities and solid tiles in the level.
pub struct TileCollisionSystem;
impl System<SystemContext<'_>> for TileCollisionSystem {
    /// Updates the system, resolving collisions for all entities with a `Position`, `Velocity`, and `Collision` component.
    fn update(&mut self, world: &mut World, context: &mut SystemContext) {
        world.grounded_tags.clear();
        let mut entities_to_ground = Vec::new();

        for (entity, pos) in world.positions.iter_mut() {
            if let (Some(vel), Some(collision)) = (world.velocities.get_mut(entity), world.collisions.get_mut(entity)) {
                collision.rect.set_x(pos.0.x as i32);
                collision.rect.set_y(pos.0.y as i32);

                let grounded = physics::resolve_vertical_collisions(pos, vel, collision.rect, context);
                physics::resolve_horizontal_collisions(pos, vel, collision.rect, context);

                if grounded {
                    entities_to_ground.push(*entity);
                }
            }
        }

        for entity in entities_to_ground {
            world.add_grounded(entity, Grounded);
        }
    }
}
