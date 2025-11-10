//! This system manages the invincibility timer for entities.

use crate::ecs::systems::{System, SystemContext};
use crate::ecs::world::World;

/// The system responsible for counting down invincibility timers.
pub struct InvincibilitySystem;

impl System<SystemContext<'_>> for InvincibilitySystem {
    /// Updates all entities with an `Invincibility` component.
    ///
    /// It decrements the timer for each entity. If an entity's timer reaches
    /// zero, the `Invincibility` component is removed, making the entity
    /// vulnerable to damage again.
    fn update(&mut self, world: &mut World, _context: &mut SystemContext) {
        let mut to_remove = Vec::new();
        for (entity, invincibility) in world.invincibilities.iter_mut() {
            // Assuming a fixed time step of 60 FPS.
            // TODO: Use a delta time from the game loop for frame-rate independence.
            invincibility.timer -= 1.0 / 60.0;
            if invincibility.timer <= 0.0 {
                to_remove.push(*entity);
            }
        }

        for entity in to_remove {
            world.invincibilities.remove(&entity);
        }
    }
}
