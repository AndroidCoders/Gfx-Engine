// src/physics.rs

//! This module contains generic physics and collision detection logic for the engine.
//! 
//! It provides functions for resolving collisions between entities and the game world.
//! 
//! # Examples
//! 
//! ```no_run
//! use crate::ecs::component::{Position, Velocity};
//! use crate::ecs::systems::SystemContext;
//! use sdl3::rect::Rect;
//! 
//! // Assuming you have a position, velocity, bounds, and context
//! // let mut pos = Position(Vector2D::new(0.0, 0.0));
//! // let mut vel = Velocity(Vector2D::new(10.0, 10.0));
//! // let bounds = Rect::new(0, 0, 32, 32);
//! // let context: SystemContext = ...;
//! // resolve_vertical_collisions(&mut pos, &mut vel, bounds, &context);
//! // resolve_horizontal_collisions(&mut pos, &mut vel, bounds, &context);
//! ```

use crate::ecs::component::{Position, Velocity};
use crate::ecs::systems::SystemContext;

/// Resolves vertical collisions between an entity and the solid tiles in the level.
///
/// It checks for collisions above and below the entity. If a collision occurs,
/// it adjusts the entity's Y position to be adjacent to the tile and resets
/// its vertical velocity.
///
/// # Arguments
///
/// * `pos` - A mutable reference to the entity's `Position` component.
/// * `vel` - A mutable reference to the entity's `Velocity` component.
/// * `bounds` - The entity's collision bounding box.
/// * `context` - The system context, providing access to level data.
///
/// # Returns
///
/// Returns `true` if the entity is grounded (i.e., a downward collision occurred),
/// `false` otherwise.
pub fn resolve_vertical_collisions(
    pos: &mut Position,
    vel: &mut Velocity,
    bounds: sdl3::rect::Rect,
    context: &SystemContext,
) -> bool {
    let tile_height = context.level.tileset.tile_height as f32;
    let tile_width = context.level.tileset.tile_width as f32;
    let scaled_bounds_width = bounds.width() as f32;
    let scaled_bounds_height = bounds.height() as f32;

    let next_y = pos.0.y + vel.0.y;
    let mut grounded = false;

    if vel.0.y > 0.0 { // Moving down
        let left_x = pos.0.x + 0.1;
        let right_x = pos.0.x + scaled_bounds_width - 0.1;
        let bottom_y = next_y + scaled_bounds_height;

        let left_tile = (left_x / tile_width).floor() as usize;
        let right_tile = (right_x / tile_width).floor() as usize;
        let bottom_tile = (bottom_y / tile_height).floor() as usize;

        if context.level.is_solid(left_tile, bottom_tile) || context.level.is_solid(right_tile, bottom_tile) {
            pos.0.y = (bottom_tile as f32 * tile_height) - scaled_bounds_height;
            vel.0.y = 0.0;
            grounded = true;
        }
    } else if vel.0.y < 0.0 { // Moving up
        let left_x = pos.0.x + 0.1;
        let right_x = pos.0.x + scaled_bounds_width - 0.1;
        let top_y = next_y;

        let left_tile = (left_x / tile_width).floor() as usize;
        let right_tile = (right_x / tile_width).floor() as usize;
        let top_tile = (top_y / tile_height).floor() as usize;

        if context.level.is_solid(left_tile, top_tile) || context.level.is_solid(right_tile, top_tile) {
            pos.0.y = (top_tile as f32 * tile_height) + tile_height;
            vel.0.y = 0.0;
        }
    }

    grounded
}

/// Resolves horizontal collisions between an entity and the solid tiles in the level.
///
/// It checks for collisions to the left and right of the entity. If a collision
/// occurs, it adjusts the entity's X position to be adjacent to the tile and
/// resets its horizontal velocity.
///
/// # Arguments
///
/// * `pos` - A mutable reference to the entity's `Position` component.
/// * `vel` - A mutable reference to the entity's `Velocity` component.
/// * `bounds` - The entity's collision bounding box.
/// * `context` - The system context, providing access to level data.
pub fn resolve_horizontal_collisions(
    pos: &mut Position,
    vel: &mut Velocity,
    bounds: sdl3::rect::Rect,
    context: &SystemContext,
) {
    let tile_height = context.level.tileset.tile_height as f32;
    let tile_width = context.level.tileset.tile_width as f32;
    let scaled_bounds_width = bounds.width() as f32;
    let scaled_bounds_height = bounds.height() as f32;

    let next_x = pos.0.x + vel.0.x;

    if vel.0.x > 0.0 { // Moving right
        let top_y = pos.0.y + 0.1;
        let bottom_y = pos.0.y + scaled_bounds_height - 0.1;
        let right_x = next_x + scaled_bounds_width;

        let top_tile = (top_y / tile_height).floor() as usize;
        let bottom_tile = (bottom_y / tile_height).floor() as usize;
        let right_tile = (right_x / tile_width).floor() as usize;

        if context.level.is_solid(right_tile, top_tile) || context.level.is_solid(right_tile, bottom_tile) {
            pos.0.x = (right_tile as f32 * tile_width) - scaled_bounds_width - 1.0;
            vel.0.x = 0.0;
        }
    } else if vel.0.x < 0.0 { // Moving left
        let top_y = pos.0.y + 0.1;
        let bottom_y = pos.0.y + scaled_bounds_height - 0.1;
        let left_x = next_x;

        let top_tile = (top_y / tile_height).floor() as usize;
        let bottom_tile = (bottom_y / tile_height).floor() as usize;
        let left_tile = (left_x / tile_width).floor() as usize;

        if context.level.is_solid(left_tile, top_tile) || context.level.is_solid(left_tile, bottom_tile) {
            pos.0.x = (left_tile as f32 * tile_width) + tile_width + 1.0;
            vel.0.x = 0.0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::level::{Level, Map, Tileset, Collision as LevelCollision};
    use crate::config::{Config, WindowConfig, InputConfig, PhysicsConfig, DebugConfig, GameConfig, PlayerConfig, WorldConfig, GameSettings};
    use crate::input::InputState;
    use crate::math::Vector2D;
    use sdl3::rect::Rect;
    use std::collections::HashMap;
    use std::sync::mpsc;

    // Helper function to create a mock level with a solid floor
    fn create_test_level_with_floor() -> Level {
        let tiles = vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![1, 1, 1, 1, 1], // Solid floor
            vec![0, 0, 0, 0, 0],
        ];
        Level {
            tileset: Tileset {
                texture: "test.png".to_string(),
                tile_width: 32,
                tile_height: 32,
            },
            map: Map { tiles: tiles.clone() },
            collision: LevelCollision { tiles },
            entities: vec![],
        }
    }

    // Helper function to create a mock level with a wall
    fn create_test_level_with_wall() -> Level {
        let tiles = vec![
            vec![0, 1, 0, 0, 0],
            vec![0, 1, 0, 0, 0],
            vec![0, 1, 0, 0, 0],
            vec![0, 1, 0, 0, 0],
            vec![0, 1, 0, 0, 0],
        ];
        Level {
            tileset: Tileset { texture: "test.png".to_string(), tile_width: 32, tile_height: 32 },
            map: Map { tiles: tiles.clone() },
            collision: LevelCollision { tiles },
            entities: vec![],
        }
    }
    
    // Helper for mock config
    fn create_test_config() -> Config {
        Config {
            window: WindowConfig { title: "".to_string(), width: 0, height: 0, virtual_width: 0, virtual_height: 0, fullscreen: false, vsync: false, scaling_quality: "".to_string(), camera_tightness: 0.0, camera_slow_zone: 0.0, camera_fast_zone: 0.0, camera_vertical_snap_threshold: 0.0, camera_vertical_tightness: 0.0, camera_falling_tightness: 0.0, camera_falling_velocity_threshold: 0.0, camera_lookahead_distance: 0.0 },
            input: InputConfig { left: "".to_string(), right: "".to_string(), jump: "".to_string(), quit: "".to_string(), debug_toggle: "".to_string() },
            physics: PhysicsConfig { gravity: 0.0, max_speed: 0.0, entity_max_fall_speed: 0.0, acceleration: 0.0, deceleration: 0.0, jump_strength: 0.0, jump_hold_force: 0.0 },
            debug: DebugConfig { show_debug_info: false, debug_draw_collision_boxes: false },
            game: GameSettings { start_level: "".to_string() },
        }
    }

    #[test]
    fn test_vertical_collision_stops_fall() {
        let level = create_test_level_with_floor();
        let config = create_test_config();
        let game_config = GameConfig {
            player: PlayerConfig { start_pos: Vector2D::default(), width: 32, height: 32, draw_width: 32, draw_height: 32, horizontal_draw_offset: 0, vertical_draw_offset: 0, respawn_pos: Vector2D::default(), lives: 3, max_health: 3 }, 
            world: WorldConfig { width: 0.0, death_plane_y: 0.0 },
            gameplay: Default::default(),
            animation: HashMap::new(), 
            audio: HashMap::new(),
            sound_events: HashMap::new(),
            textures: HashMap::new(),
            prefabs: HashMap::new(),
        };
        let input_state = InputState::default();
        let (sender, _) = mpsc::channel();
        let mut gold_coin_count = 0;
        let mut lives = 3;
        let mut next_level = None;

        let context = SystemContext {
            level: &level,
            input_state: &input_state,
            config: &config,
            game_config: &game_config,
            audio_sender: &sender,
            gold_coin_count: &mut gold_coin_count,
            lives: &mut lives,
            next_level: &mut next_level,
            delta_time: 0.016,
        };

        let mut pos = Position(Vector2D::new(32.0, 60.0)); // Start above the floor
        let mut vel = Velocity(Vector2D::new(0.0, 10.0)); // Falling down
        let bounds = Rect::new(pos.0.x as i32, pos.0.y as i32, 32, 32);

        let grounded = resolve_vertical_collisions(&mut pos, &mut vel, bounds, &context);

        assert!(grounded, "Player should be grounded after collision");
        assert_eq!(pos.0.y, 64.0, "Player should be positioned on top of the floor"); // 3 * 32 (tile_y) - 32 (player_height)
        assert_eq!(vel.0.y, 0.0, "Player vertical velocity should be zero after collision");
    }

    #[test]
    fn test_horizontal_collision_stops_movement() {
        let level = create_test_level_with_wall();
        let config = create_test_config();
        let game_config = GameConfig {
            player: PlayerConfig { start_pos: Vector2D::default(), width: 32, height: 32, draw_width: 32, draw_height: 32, horizontal_draw_offset: 0, vertical_draw_offset: 0, respawn_pos: Vector2D::default(), lives: 3, max_health: 3 }, 
            world: WorldConfig { width: 0.0, death_plane_y: 0.0 },
            gameplay: Default::default(),
            animation: HashMap::new(), 
            audio: HashMap::new(),
            sound_events: HashMap::new(),
            textures: HashMap::new(),
            prefabs: HashMap::new(),
        };
        let input_state = InputState::default();
        let (sender, _) = mpsc::channel();
        let mut gold_coin_count = 0;
        let mut lives = 3;
        let mut next_level = None;

        let context = SystemContext {
            level: &level,
            input_state: &input_state,
            config: &config,
            game_config: &game_config,
            audio_sender: &sender,
            gold_coin_count: &mut gold_coin_count,
            lives: &mut lives,
            next_level: &mut next_level,
            delta_time: 0.016,
        };

        let mut pos = Position(Vector2D::new(0.0, 64.0)); // Start to the left of the wall
        let mut vel = Velocity(Vector2D::new(5.0, 0.0)); // Moving right
        let bounds = Rect::new(pos.0.x as i32, pos.0.y as i32, 32, 32);

        resolve_horizontal_collisions(&mut pos, &mut vel, bounds, &context);

        assert_eq!(pos.0.x, -1.0, "Player should be positioned just to the left of the wall");
        assert_eq!(vel.0.x, 0.0, "Player horizontal velocity should be zero after collision");
    }
}