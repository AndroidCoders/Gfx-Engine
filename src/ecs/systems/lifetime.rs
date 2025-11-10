//! This system manages entities with a limited lifetime.

use crate::ecs::component::DeadTag;
use crate::ecs::systems::{System, SystemContext};
use crate::ecs::world::World;

/// The system that handles entities with a `Lifetime` component.
pub struct LifetimeSystem;

impl System<SystemContext<'_>> for LifetimeSystem {
    /// Updates all entities with a `Lifetime` component.
    ///
    /// It decrements the timer for each entity. If an entity's timer reaches
    /// zero, it is tagged with a `DeadTag` to be removed by the `KillSystem`.
    /// This is useful for temporary effects like explosions.
    fn update(&mut self, world: &mut World, _context: &mut SystemContext) {
        let mut to_kill = Vec::new();
        for (entity, lifetime) in world.lifetimes.iter_mut() {
            // Assuming a fixed time step of 60 FPS.
            // TODO: Use a delta time from the game loop for frame-rate independence.
            lifetime.timer -= 1.0 / 60.0;
            if lifetime.timer <= 0.0 {
                to_kill.push(*entity);
            }
        }

        for entity in to_kill {
            world.add_dead_tag(entity, DeadTag);
        }
    }
}
