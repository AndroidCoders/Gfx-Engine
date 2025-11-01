// src/ecs/systems/lifetime.rs

use crate::ecs::component::{DeadTag, Lifetime};
use crate::ecs::systems::{System, SystemContext};
use crate::ecs::world::World;

pub struct LifetimeSystem;

impl System<SystemContext<'_>> for LifetimeSystem {
    fn update(&mut self, world: &mut World, _context: &mut SystemContext) {
        let mut to_kill = Vec::new();
        for (entity, lifetime) in world.lifetimes.iter_mut() {
            lifetime.timer -= 1.0 / 60.0; // Assuming 60 FPS
            if lifetime.timer <= 0.0 {
                to_kill.push(*entity);
            }
        }

        for entity in to_kill {
            world.add_dead_tag(entity, DeadTag);
        }
    }
}
