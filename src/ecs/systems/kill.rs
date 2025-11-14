use crate::ecs::systems::{System, SystemContext};
use crate::ecs::world::{Entity, World};

pub struct KillSystem;
impl System<SystemContext<'_>> for KillSystem {
    fn update(&mut self, world: &mut World, _context: &mut SystemContext) {
        let dead_entities: Vec<Entity> = world.dead_tags.keys().copied().collect();
        for entity in dead_entities {
            world.positions.remove(&entity);
            world.velocities.remove(&entity);
            world.renderables.remove(&entity);
            world.animations.remove(&entity);
            world.enemy_tags.remove(&entity);
            world.gold_coins.remove(&entity);
            world.dead_tags.remove(&entity);
            world.patrols.remove(&entity);
            world.gravity_tags.remove(&entity);
            world.collisions.remove(&entity);
            world.grounded_tags.remove(&entity);
            world.state_components.remove(&entity);
        }
    }
}
