// src/ecs/systems/player_death.rs

use crate::ecs::component::RespawnTag;
use crate::ecs::systems::{System, SystemContext};
use crate::ecs::world::World;

pub struct PlayerDeathSystem;

impl System<SystemContext<'_>> for PlayerDeathSystem {
    fn update(&mut self, world: &mut World, _context: &mut SystemContext) {
        let mut to_respawn = Vec::new();
        for (entity, health) in &world.healths {
            if health.current == 0 {
                // Check if the entity is a player before marking for respawn
                if world.player_tags.contains_key(entity) {
                    to_respawn.push(*entity);
                }
            }
        }

        for entity in to_respawn {
            world.add_respawn_tag(entity, RespawnTag);
            // Also reset health for the next life
            if let Some(health) = world.healths.get_mut(&entity) {
                health.current = health.max;
            }
        }
    }
}
