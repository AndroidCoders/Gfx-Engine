//! # Concept: Tile Collision Resolution
//! 
//! This module acts as the authoritative resolver for environment constraints.
//! It detects overlaps between entities and the static level geometry (solid tiles)
//! and enforces physical boundaries by adjusting positions.

use crate::ecs::component::{Grounded, WallHit};
use crate::ecs::systems::{System, SystemContext};
use crate::physics;

/// A system that resolves entity positions against solid tiles and identifies surface contact.
pub struct SystemTileCollision;

impl System<SystemContext<'_>> for SystemTileCollision {
    /// Resolves entity positions against geometry and updates grounded/wall-hit facts.
    fn update(&mut self, world: &mut crate::ecs::world::World, context: &mut SystemContext) {
        // 1. Reset state tags from the previous frame.
        world.grounded_tags.clear();
        world.wall_hits.clear();
        
        let mut entities_to_ground = Vec::new();
        let mut entities_hit_wall = Vec::new();

        for (entity, pos) in &mut world.positions {
            // 2. Only process entities with movement (Velocity) and physical bounds (Collision).
            if let (Some(vel), Some(collision)) = (world.velocities.get_mut(entity), world.collisions.get_mut(entity)) {
                // 3. Sync collision rect with current position before checking tiles.
                collision.rect.set_x(pos.0.x as i32);
                collision.rect.set_y(pos.0.y as i32);

                // 4. Resolve Vertical Collisions (Gravity/Jumping vs Floors/Ceilings).
                let grounded = physics::resolve_vertical_collisions(pos, vel, collision.rect, context);
                
                // 5. Resolve Horizontal Collisions (Walking vs Walls).
                let wall_hit = physics::resolve_horizontal_collisions(pos, vel, collision.rect, context);

                // 6. Buffer the results to avoid simultaneous mutable borrow of the World.
                if grounded {
                    entities_to_ground.push(*entity);
                }
                if let Some(normal) = wall_hit {
                    entities_hit_wall.push((*entity, normal));
                }
            }
        }

        // 7. Apply the derived state tags back to the entities.
        for entity in entities_to_ground {
            world.add_grounded(entity, Grounded);
        }
        for (entity, normal) in entities_hit_wall {
            world.add_wall_hit(entity, WallHit { normal_x: normal });
        }
    }
}
