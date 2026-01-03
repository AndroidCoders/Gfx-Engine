//! # Concept: Enemy Behavior (Patrol)
//! 
//! This module defines the logical states for automated enemies. 
//! It provides the 'Patrol' behavior, where an entity moves horizontally 
//! and automatically reverses direction when it detects physical obstacles 
//! or upcoming platform ledges.

use crate::state_machine::State;
use crate::ecs::world::{World, Entity};
use crate::ecs::systems::SystemContext;
use crate::ecs::component::MovementIntention;

/// The primary state for ground-based automated enemies.
pub struct PatrolState;

impl PatrolState {
    pub fn new() -> Self { Self }
}

impl Default for PatrolState {
    fn default() -> Self {
        Self::new()
    }
}

impl State for PatrolState {
    fn enter(&mut self) {}
    fn exit(&mut self) {}

    /// Applies the current patrol intention based on the entity's movement direction.
    fn update_with_context(&mut self, world: &mut World, _context: &mut SystemContext, entity: Entity) {
        // 1. Retrieve the entity's current intended movement direction.
        let dir = if let Some(patrol) = world.patrols.get(&entity) {
            patrol.direction
        } else {
            1.0
        };
        
        // 2. Publish a movement intention to the motor system.
        world.add_movement_intention(entity, MovementIntention { x: dir });
    }

    /// Evaluates environmental constraints to determine when to reverse movement direction.
    fn transition_with_context(&mut self, world: &mut World, context: &mut SystemContext, entity: Entity) -> Option<Box<dyn State>> {
        let is_grounded = world.is_grounded(entity);
        
        let mut should_reverse = false;
        let mut current_dir = 1.0;
        let mut enemy_width = 32.0;
        let mut enemy_height = 32.0;
        let mut pos_x = 0.0;
        let mut pos_y = 0.0;
        
        // 1. Gather the physical state required for environmental probing.
        if let Some(pos) = world.positions.get(&entity) {
            pos_x = pos.0.x;
            pos_y = pos.0.y;
        }
        
        if let Some(collision) = world.collisions.get(&entity) {
            enemy_width = collision.rect.width() as f32;
            enemy_height = collision.rect.height() as f32;
        }
        
        if let Some(patrol) = world.patrols.get(&entity) {
            current_dir = patrol.direction;
        }

        let tile_width = context.level.tileset.tile_width as f32;
        let tile_height = context.level.tileset.tile_height as f32;

        // 2. Check for authoritative wall hits reported by the physics engine (Priority 1).
        if let Some(wall_hit) = world.wall_hits.get(&entity) {
            should_reverse = true;
            if let Some(patrol) = world.patrols.get_mut(&entity) {
                patrol.direction = wall_hit.normal_x;
            }
        } 
        else {
            // 3. No wall hit. Check environmental triggers (Priority 2).
            
            // A. Map Boundaries (Predictive)
            let wall_check_x = if current_dir > 0.0 {
                pos_x + enemy_width + 4.0
            } else {
                pos_x - 4.0
            };

            let map_width = context.level.map.tiles[0].len() as f32 * tile_width;
            
            if wall_check_x < 0.0 || wall_check_x > map_width {
                should_reverse = true;
            } 
            else if is_grounded {
                // B. Ledges (Only if grounded and not at map edge)
                let check_x = if current_dir > 0.0 {
                    pos_x + enemy_width + 1.0
                } else {
                    pos_x - 1.0
                };

                // Check one pixel below feet to detect upcoming ledges.
                let ground_check_y = pos_y + enemy_height + 1.0;
                let ground_tile_x = (check_x / tile_width).floor() as usize;
                let ground_tile_y = (ground_check_y / tile_height).floor() as usize;

                if !context.level.is_solid(ground_tile_x, ground_tile_y) {
                    should_reverse = true;
                }
            }
        }

        // 4. Apply reversal logic: zero velocity and flip direction.
        if should_reverse {
            if let Some(vel) = world.velocities.get_mut(&entity) {
                vel.0.x = 0.0; 
            }
            
            // Only flip blindly if we didn't already set the direction via WallHit.
            if !world.wall_hits.contains_key(&entity) && let Some(patrol) = world.patrols.get_mut(&entity) {
                patrol.direction *= -1.0;
            }
        }
        
        None 
    }

    fn get_name(&self) -> &str { "PatrolState" }
}