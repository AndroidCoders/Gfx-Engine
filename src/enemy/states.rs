// src/enemy/states.rs

//! Defines the enemy's specific states and the logic for each state.

use crate::state_machine::State;
use crate::ecs::world::{World, Entity};
use crate::ecs::system::SystemContext;

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
        if let Some(vel) = world.velocities.get_mut(&entity) {
            if let Some(pos) = world.positions.get(&entity) {
                let tile_width = context.level.tileset.tile_width as f32;
                let next_x = pos.0.x + vel.0.x;

                // Wall check
                let top_y = pos.0.y + 0.1;
                let bottom_y = pos.0.y + context.game_config.enemy["slime"].height as f32 - 0.1;
                let right_x = next_x + context.game_config.enemy["slime"].width as f32;
                let left_x = next_x;

                let top_tile_y = (top_y / context.level.tileset.tile_height as f32).floor() as usize;
                let bottom_tile_y = (bottom_y / context.level.tileset.tile_height as f32).floor() as usize;
                let right_tile_x = (right_x / tile_width).floor() as usize;
                let left_tile_x = (left_x / tile_width).floor() as usize;

                if vel.0.x > 0.0 { // Moving right
                    if context.level.is_solid(right_tile_x, top_tile_y) || context.level.is_solid(right_tile_x, bottom_tile_y) {
                        vel.0.x *= -1.0;
                    }
                } else { // Moving left
                    if context.level.is_solid(left_tile_x, top_tile_y) || context.level.is_solid(left_tile_x, bottom_tile_y) {
                        vel.0.x *= -1.0;
                    }
                }
            }
        }
        None // Always stay in PatrolState for now
    }

    fn get_name(&self) -> &str {
        "PatrolState"
    }
}
