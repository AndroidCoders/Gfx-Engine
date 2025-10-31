use crate::ecs::systems::{System, SystemContext};
use crate::ecs::world::World;

pub struct AnimationUpdateSystem;
impl System<SystemContext<'_>> for AnimationUpdateSystem {
    fn update(&mut self, world: &mut World, _context: &mut SystemContext) {
        for (_, animation) in world.animations.iter_mut() {
            animation.controller.update();
        }
    }
}
