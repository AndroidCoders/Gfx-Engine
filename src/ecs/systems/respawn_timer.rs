use crate::ecs::systems::{System, SystemContext};
use crate::ecs::world::{World};

pub struct RespawnTimerSystem;
impl System<SystemContext<'_>> for RespawnTimerSystem {
    fn update(&mut self, world: &mut World, _context: &mut SystemContext) {
        let mut to_remove = Vec::new();
        for (entity, timer) in world.respawn_timers.iter_mut() {
            // Assuming a fixed time step for simplicity for now
            // A better approach would be to use a delta time from the game loop
            timer.timer -= 1.0 / 60.0; // Assuming 60 FPS
            if timer.timer <= 0.0 {
                to_remove.push(*entity);
            }
        }

        for entity in to_remove {
            world.respawn_timers.remove(&entity);
        }
    }
}
