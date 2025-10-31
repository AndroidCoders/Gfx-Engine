use crate::ecs::component::RespawnTag;
use crate::ecs::systems::{System, SystemContext};
use crate::ecs::world::World;

pub struct DeathSystem;
impl System<SystemContext<'_>> for DeathSystem {
    fn update(&mut self, world: &mut World, context: &mut SystemContext) {
        let death_plane_y = context.game_config.world.death_plane_y;
        let mut to_respawn = Vec::new();

        for (entity, _player_tag) in world.player_tags.iter() {
            // Ignore players with a respawn timer (grace period)
            if world.respawn_timers.contains_key(entity) {
                continue;
            }

            if let Some(pos) = world.positions.get(entity) {
                if pos.0.y > death_plane_y {
                    to_respawn.push(*entity);
                }
            }
        }

        for entity in to_respawn {
            world.add_respawn_tag(entity, RespawnTag);
        }
    }
}
