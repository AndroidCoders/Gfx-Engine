//! # Concept: Physics Integration
//! 
//! This module is the engine's core Euler integrator. 
//! It is responsible for the pure mathematical task of advancing velocity 
//! based on forces (Gravity) and intended movement (Acceleration).

use crate::ecs::systems::{System, SystemContext};

/// A system that integrates forces and acceleration into entity velocities.
pub struct SystemPhysics;

impl System<SystemContext<'_>> for SystemPhysics {
    /// Applies global forces and local acceleration to update entity velocities.
    fn update(&mut self, world: &mut crate::ecs::world::World, context: &mut SystemContext<'_>) {
        // 1. Apply Gravity (Global Constant Force) to all affected entities.
        for (entity, _gravity) in world.gravity_tags.iter() {
            // Skip entities outside the active simulation range.
            if world.is_dormant(*entity) { continue; }

            if let Some(vel) = world.velocities.get_mut(entity) {
                // Integrate gravity into vertical velocity.
                vel.0.y += context.config.physics.gravity * context.delta_time;
                // Enforce terminal velocity to prevent tunneling through thin platforms.
                vel.0.y = vel.0.y.min(context.config.physics.entity_max_fall_speed);
            }
        }

        // 2. Integrate the local Acceleration component into Velocity.
        for (entity, accel) in world.accelerations.iter() {
            // Skip entities outside the active simulation range.
            if world.is_dormant(*entity) { continue; }

            if let Some(vel) = world.velocities.get_mut(entity) {
                // Integrate linear acceleration (scaled by delta time).
                vel.0.x += accel.0.x * context.delta_time;
                vel.0.y += accel.0.y * context.delta_time;
            }
        }
    }
}
