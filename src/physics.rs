// src/physics.rs

//! Contains generic physics and collision detection logic.

use crate::ecs::component::{Position, Velocity};
use crate::ecs::systems::SystemContext;

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
    use crate::config::{Config, WindowConfig, InputConfig, PhysicsConfig, DebugConfig, GameConfig, PlayerConfig, WorldConfig};
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
            window: WindowConfig { title: "".to_string(), width: 0, height: 0, virtual_width: 0, virtual_height: 0, fullscreen: false, vsync: false, scaling_quality: "".to_string(), camera_tightness: 0.0, camera_slow_zone: 0.0, camera_fast_zone: 0.0, camera_vertical_snap_threshold: 0.0, camera_vertical_tightness: 0.0, camera_falling_tightness: 0.0, camera_falling_velocity_threshold: 0.0 },
            input: InputConfig { left: "".to_string(), right: "".to_string(), jump: "".to_string(), quit: "".to_string(), debug_toggle: "".to_string() },
            physics: PhysicsConfig { gravity: 0.0, max_speed: 0.0, entity_max_fall_speed: 0.0, acceleration: 0.0, friction: 0.0, jump_strength: 0.0, jump_hold_force: 0.0 },
            debug: DebugConfig { show_debug_info: false, debug_draw_collision_boxes: false },
        }
    }

    #[test]
    fn test_vertical_collision_stops_fall() {
        let level = create_test_level_with_floor();
        let config = create_test_config();
        let game_config = GameConfig { player: PlayerConfig { start_pos: Vector2D::default(), width: 32, height: 32, draw_width: 32, draw_height: 32, horizontal_draw_offset: 0, vertical_draw_offset: 0, respawn_pos: Vector2D::default() }, world: WorldConfig { width: 0.0, death_plane_y: 0.0 }, enemy: HashMap::new(), collectible: HashMap::new(), animation: HashMap::new() };
        let input_state = InputState::default();
        let (sender, _) = mpsc::channel();
        let mut gold_coin_count = 0;

        let context = SystemContext {
            level: &level,
            input_state: &input_state,
            config: &config,
            game_config: &game_config,
            audio_sender: &sender,
            gold_coin_count: &mut gold_coin_count,
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
        let game_config = GameConfig { player: PlayerConfig { start_pos: Vector2D::default(), width: 32, height: 32, draw_width: 32, draw_height: 32, horizontal_draw_offset: 0, vertical_draw_offset: 0, respawn_pos: Vector2D::default() }, world: WorldConfig { width: 0.0, death_plane_y: 0.0 }, enemy: HashMap::new(), collectible: HashMap::new(), animation: HashMap::new() };
        let input_state = InputState::default();
        let (sender, _) = mpsc::channel();
        let mut gold_coin_count = 0;

        let context = SystemContext {
            level: &level,
            input_state: &input_state,
            config: &config,
            game_config: &game_config,
            audio_sender: &sender,
            gold_coin_count: &mut gold_coin_count,
        };

        let mut pos = Position(Vector2D::new(0.0, 64.0)); // Start to the left of the wall
        let mut vel = Velocity(Vector2D::new(5.0, 0.0)); // Moving right
        let bounds = Rect::new(pos.0.x as i32, pos.0.y as i32, 32, 32);

        resolve_horizontal_collisions(&mut pos, &mut vel, bounds, &context);

        assert_eq!(pos.0.x, -1.0, "Player should be positioned just to the left of the wall");
        assert_eq!(vel.0.x, 0.0, "Player horizontal velocity should be zero after collision");
    }
}
