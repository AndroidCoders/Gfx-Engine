// src/physics.rs

//! Contains generic physics and collision detection logic.

use crate::ecs::component::{Position, Velocity};
use crate::ecs::system::SystemContext;

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
