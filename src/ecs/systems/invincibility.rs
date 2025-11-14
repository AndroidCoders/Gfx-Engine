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
    fn update(&mut self, world: &mut World, context: &mut SystemContext) {
        let mut to_remove = Vec::new();
        for (entity, invincibility) in world.invincibilities.iter_mut() {
            invincibility.timer -= context.delta_time;
            if invincibility.timer <= 0.0 {
                to_remove.push(*entity);
            }
        }

        for entity in to_remove {
            world.invincibilities.remove(&entity);
        }
    }
}
