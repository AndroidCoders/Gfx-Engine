use crate::ecs::systems::{System, SystemContext};
use crate::ecs::world::World;

pub struct PhysicsSystem;
impl System<SystemContext<'_>> for PhysicsSystem {
    fn update(&mut self, world: &mut World, context: &mut SystemContext) {
        for (entity, _gravity) in world.gravity_tags.iter() {
            if let Some(vel) = world.velocities.get_mut(entity) {
                vel.0.y += context.config.physics.gravity;
                vel.0.y = vel.0.y.min(context.config.physics.entity_max_fall_speed);
            }
        }

        for (entity, pos) in world.positions.iter_mut() {
            if let Some(vel) = world.velocities.get(entity) {
                pos.0.x += vel.0.x;
                pos.0.y += vel.0.y;
            }
        }
    }
}
