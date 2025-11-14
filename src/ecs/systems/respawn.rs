//! This system handles the respawning of entities.

use crate::ecs::component::RespawnTimer;
use crate::ecs::systems::{RespawnSystemContext, System};
use crate::ecs::world::{Entity, World};

/// The system responsible for processing entity respawns.
pub struct RespawnSystem;
impl System<RespawnSystemContext<'_>> for RespawnSystem {
    /// Finds all entities with a `RespawnTag` and resets their state.
    ///
    /// For each entity to respawn, it:
    /// 1. Removes the `RespawnTag`.
    /// 2. Resets the entity's position to the configured respawn point.
    /// 3. Snaps the camera to the new respawn position.
    /// 4. Resets the entity's velocity to zero.
    /// 5. Adds a `RespawnTimer` component to grant temporary invincibility.
    fn update(&mut self, world: &mut World, context: &mut RespawnSystemContext) {
        let respawn_pos = context.game_config.player.respawn_pos;

        let to_respawn: Vec<Entity> = world.respawn_tags.keys().copied().collect();

        for entity in to_respawn {
            world.respawn_tags.remove(&entity);

            if let Some(pos) = world.positions.get_mut(&entity) {
                pos.0 = respawn_pos;
                // Reset camera position
                context.camera.snap_to(respawn_pos);
            }
            if let Some(vel) = world.velocities.get_mut(&entity) {
                vel.0.x = 0.0;
                vel.0.y = 0.0;
            }

            // Add a respawn timer for a grace period (e.g., 2 seconds)
            world.add_respawn_timer(entity, RespawnTimer { timer: 2.0 });
        }
    }
}
