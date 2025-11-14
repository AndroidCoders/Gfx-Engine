// src/ecs/systems/invincibility.rs

use crate::ecs::systems::{System, SystemContext};
use crate::ecs::world::World;

pub struct InvincibilitySystem;

impl System<SystemContext<'_>> for InvincibilitySystem {
    fn update(&mut self, world: &mut World, _context: &mut SystemContext) {
        let mut to_remove = Vec::new();
        for (entity, invincibility) in world.invincibilities.iter_mut() {
            invincibility.timer -= 1.0 / 60.0; // Assuming 60 FPS
            if invincibility.timer <= 0.0 {
                to_remove.push(*entity);
            }
        }

        for entity in to_remove {
            world.invincibilities.remove(&entity);
        }
    }
}
