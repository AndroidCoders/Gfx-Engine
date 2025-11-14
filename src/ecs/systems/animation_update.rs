//! This system is responsible for advancing the animation frames for all entities.

use crate::ecs::systems::{System, SystemContext};
use crate::ecs::world::World;

/// The system that updates all animation controllers.
pub struct AnimationUpdateSystem;
impl System<SystemContext<'_>> for AnimationUpdateSystem {
    /// Iterates through all entities with an `Animation` component and calls
    /// the `update` method on their `AnimationController`.
    fn update(&mut self, world: &mut World, _context: &mut SystemContext) {
        for (_, animation) in world.animations.iter_mut() {
            animation.controller.update();
        }
    }
}
