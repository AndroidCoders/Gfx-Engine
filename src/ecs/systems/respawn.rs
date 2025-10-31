use crate::ecs::component::RespawnTimer;
use crate::ecs::systems::{RespawnSystemContext, System};
use crate::ecs::world::{Entity, World};

pub struct RespawnSystem;
impl System<RespawnSystemContext<'_>> for RespawnSystem {
    fn update(&mut self, world: &mut World, context: &mut RespawnSystemContext) {
        let respawn_pos = context.game_config.player.respawn_pos;

        let to_respawn: Vec<Entity> = world.respawn_tags.keys().copied().collect();

        for entity in to_respawn {
            world.respawn_tags.remove(&entity);

            if let Some(pos) = world.positions.get_mut(&entity) {
                pos.0 = respawn_pos;
                // Reset camera position
                context.camera.position.x = respawn_pos.x - (context.camera.virtual_width / 2.0);
                context.camera.position.y = respawn_pos.y - (context.camera.virtual_height / 2.0);
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
