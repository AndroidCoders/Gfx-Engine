//! This system handles player death when they fall out of the world.

use crate::ecs::component::RespawnTag;
use crate::ecs::systems::{System, SystemContext};
use crate::ecs::world::World;

/// The system that checks for player death by falling.
pub struct DeathSystem;
impl System<SystemContext<'_>> for DeathSystem {
    /// Checks if the player's Y position is below the configured `death_plane_y`.
    ///
    /// If the player has fallen out of the world and is not currently in a
    /// respawn grace period, they are tagged with a `RespawnTag` to trigger
    /// the respawn process.
    fn update(&mut self, world: &mut World, context: &mut SystemContext) {
        let death_plane_y = context.game_config.world.death_plane_y;
        let mut to_respawn = Vec::new();

        for (entity, _player_tag) in world.player_tags.iter() {
            // Ignore players with a respawn timer (grace period)
            if world.respawn_timers.contains_key(entity) {
                continue;
            }

            if let Some(pos) = world.positions.get(entity)
                && pos.0.y > death_plane_y {
                    to_respawn.push(*entity);
                }
        }

        for entity in to_respawn {
            world.add_respawn_tag(entity, RespawnTag);
            if *context.lives > 0 {
                *context.lives -= 1;
            }
        }
    }
}
