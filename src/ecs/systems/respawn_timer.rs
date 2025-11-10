//! This system manages the countdown for respawn invincibility timers.

use crate::ecs::systems::{System, SystemContext};
use crate::ecs::world::{World};

/// The system responsible for counting down `RespawnTimer` components.
pub struct RespawnTimerSystem;
impl System<SystemContext<'_>> for RespawnTimerSystem {
    /// Updates all entities with a `RespawnTimer` component.
    ///
    /// It decrements the timer for each entity. If an entity's timer reaches
    /// zero, the `RespawnTimer` component is removed, indicating the end
    /// of the respawn invincibility period.
    fn update(&mut self, world: &mut World, _context: &mut SystemContext) {
        let mut to_remove = Vec::new();
        for (entity, timer) in world.respawn_timers.iter_mut() {
            // Assuming a fixed time step of 60 FPS for simplicity for now.
            // TODO: A better approach would be to use a delta time from the game loop for frame-rate independence.
            timer.timer -= 1.0 / 60.0;
            if timer.timer <= 0.0 {
                to_remove.push(*entity);
            }
        }

        for entity in to_remove {
            world.respawn_timers.remove(&entity);
        }
    }
}
