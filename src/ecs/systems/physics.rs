//! This system applies basic physics rules to entities.

use crate::ecs::systems::{System, SystemContext};
use crate::ecs::world::World;

/// The system responsible for applying gravity and updating entity positions based on velocity.
pub struct PhysicsSystem;
impl System<SystemContext<'_>> for PhysicsSystem {
    /// Applies gravity to entities with a `Gravity` component and updates
    /// the position of all entities based on their velocity.
    ///
    /// Gravity is applied by increasing the vertical velocity (`vel.0.y`)
    /// up to a maximum fall speed. Positions are then updated by adding
    /// the current velocity to the position.
    fn update(&mut self, world: &mut World, context: &mut SystemContext) {
        for (entity, _gravity) in world.gravity_tags.iter() {
            if let Some(vel) = world.velocities.get_mut(entity) {
                vel.0.y += context.config.physics.gravity;
                vel.0.y = vel.0.y.min(context.config.physics.entity_max_fall_speed);
            }
        }

        for (entity, pos) in world.positions.iter_mut() {
            if let Some(vel) = world.velocities.get(entity) {
                pos.0.x += vel.0.x;
                pos.0.y += vel.0.y;
            }
        }
    }
}
