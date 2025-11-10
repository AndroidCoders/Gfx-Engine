// src/enemy/states.rs

//! Defines the enemy's specific states and the logic for each state.

use crate::state_machine::State;
use crate::ecs::world::{World, Entity};
use crate::ecs::systems::SystemContext;

/// Represents the patrolling state for an enemy.
///
/// In this state, the enemy moves horizontally until it encounters a wall
/// or the edge of a platform, at which point it reverses direction.
pub struct PatrolState;

impl State for PatrolState {
    /// Called when the enemy enters the `PatrolState`.
    fn enter(&mut self) {
        // println!("Entering PatrolState"); // Debug print
    }

    /// Called when the enemy exits the `PatrolState`.
    fn exit(&mut self) {
        // println!("Exiting PatrolState"); // Debug print
    }

    /// Updates the enemy's velocity based on its patrol speed.
    ///
    /// If the enemy's horizontal velocity is zero, it is set to the configured
    /// patrol speed. The actual movement is handled by the `PhysicsSystem`.
    fn update_with_context(&mut self, world: &mut World, _context: &mut SystemContext, entity: Entity) {
        if let Some(patrol) = world.patrols.get(&entity)
            && let Some(vel) = world.velocities.get_mut(&entity)
                && vel.0.x == 0.0 {
                    vel.0.x = patrol.speed;
                }
    }

    /// Checks for conditions that would cause the enemy to change its patrol direction.
    ///
    /// This includes detecting walls and the edges of platforms. If a wall or
    /// edge is detected, the enemy's horizontal velocity is reversed.
    /// Currently, enemies always remain in the `PatrolState`.
    fn transition_with_context(&mut self, world: &mut World, context: &mut SystemContext, entity: Entity) -> Option<Box<dyn State>> {
        let is_grounded = world.is_grounded(entity);

        if let Some(vel) = world.velocities.get_mut(&entity)
            && let Some(pos) = world.positions.get(&entity) {
                let tile_width = context.level.tileset.tile_width as f32;
                let tile_height = context.level.tileset.tile_height as f32;
                // Assuming "enemy_spider" is the default enemy type for now.
                // TODO: Make this more generic for different enemy types.
                let enemy_width = context.game_config.enemy["enemy_spider"].width as f32;
                let enemy_height = context.game_config.enemy["enemy_spider"].height as f32;

                let mut should_reverse = false;

                // Check for platform edges if grounded
                if is_grounded {
                    let check_x = if vel.0.x > 0.0 {
                        // Probe is at the front-right corner
                        pos.0.x + enemy_width
                    } else {
                        // Probe is at the front-left corner
                        pos.0.x
                    };

                    // Check one pixel below the enemy's feet to detect platform edge
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
        None // Always stay in PatrolState for now
    }

    /// Returns the name of this state for debugging purposes.
    fn get_name(&self) -> &str {
        "PatrolState"
    }
}
