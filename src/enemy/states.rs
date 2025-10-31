// src/enemy/states.rs

//! Defines the enemy's specific states and the logic for each state.

use crate::state_machine::State;
use crate::ecs::world::{World, Entity};
use crate::ecs::systems::SystemContext;

pub struct PatrolState;

impl State for PatrolState {
    fn enter(&mut self) {
        // println!("Entering PatrolState");
    }

    fn exit(&mut self) {
        // println!("Exiting PatrolState");
    }

    fn update_with_context(&mut self, world: &mut World, _context: &mut SystemContext, entity: Entity) {
        // The PhysicsSystem will move the entity based on its velocity.
        // The transition logic will handle changing direction.
        if let Some(patrol) = world.patrols.get(&entity) {
            if let Some(vel) = world.velocities.get_mut(&entity) {
                if vel.0.x == 0.0 {
                    vel.0.x = patrol.speed;
                }
            }
        }
    }

    fn transition_with_context(&mut self, world: &mut World, context: &mut SystemContext, entity: Entity) -> Option<Box<dyn State>> {
        let is_grounded = world.is_grounded(entity);

        if let Some(vel) = world.velocities.get_mut(&entity) {
            if let Some(pos) = world.positions.get(&entity) {
                let tile_width = context.level.tileset.tile_width as f32;
                let tile_height = context.level.tileset.tile_height as f32;
                let enemy_width = context.game_config.enemy["enemy_spider"].width as f32;
                let enemy_height = context.game_config.enemy["enemy_spider"].height as f32;

                let mut should_reverse = false;

                if is_grounded {
                    let check_x = if vel.0.x > 0.0 {
                        // Probe is at the front-right corner
                        pos.0.x + enemy_width
                    } else {
                        // Probe is at the front-left corner
                        pos.0.x
                    };

                    let ground_check_y = pos.0.y + enemy_height + 1.0;

                    let ground_tile_x = (check_x / tile_width).floor() as usize;
                    let ground_tile_y = (ground_check_y / tile_height).floor() as usize;

                    if !context.level.is_solid(ground_tile_x, ground_tile_y) {
                        should_reverse = true;
                    }
                }

                // --- Wall Detection ---
                if !should_reverse {
                    let next_x = pos.0.x + vel.0.x;
                    let wall_check_x = if vel.0.x > 0.0 {
                        next_x + enemy_width
                    } else {
                        next_x
                    };
                    let wall_tile_x = (wall_check_x / tile_width).floor() as usize;
                    let wall_tile_y = (pos.0.y / tile_height).floor() as usize; // Check at head height

                    if context.level.is_solid(wall_tile_x, wall_tile_y) {
                        should_reverse = true;
                    }
                }

                if should_reverse {
                    vel.0.x *= -1.0;
                }
            }
        }
        None // Always stay in PatrolState for now
    }

    fn get_name(&self) -> &str {
        "PatrolState"
    }
}
