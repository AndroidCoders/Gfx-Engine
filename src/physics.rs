//! # Concept: Environment Physics
//! 
//! This module provides the engine's physical constraint solver. it is 
//! responsible for the pure mathematical task of resolving entity positions 
//! against the static level geometry (tiles) and world boundaries.

use crate::ecs::component::{Position, Velocity};
use crate::ecs::systems::SystemContext;

/// Resolves vertical constraints and identifies surface contact.
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

    // skin width to prevent snagging on perpendicular surfaces.
    let collision_nudge = 0.2;

    // 1. Calculate the intended Y position for the next tick.
    let next_y = pos.0.y + vel.0.y * context.delta_time;
    let mut grounded = false;

    if vel.0.y > 0.0 { 
        // 2. Resolve downward movement (Falling/Landing).
        let left_x = pos.0.x + collision_nudge;
        let right_x = pos.0.x + scaled_bounds_width - collision_nudge;
        let bottom_y = next_y + scaled_bounds_height;

        let left_tile = (left_x / tile_width).floor() as usize;
        let right_tile = (right_x / tile_width).floor() as usize;
        let bottom_tile = (bottom_y / tile_height).floor() as usize;

        if context.level.is_solid(left_tile, bottom_tile) || context.level.is_solid(right_tile, bottom_tile) {
            // Snap to the surface of the floor tile and zero vertical velocity.
            pos.0.y = (bottom_tile as f32 * tile_height) - scaled_bounds_height;
            vel.0.y = 0.0;
            grounded = true;
        } else {
            pos.0.y = next_y;
        }
    } else if vel.0.y < 0.0 { 
        // 3. Resolve upward movement (Jumping/Bonking).
        let left_x = pos.0.x + collision_nudge;
        let right_x = pos.0.x + scaled_bounds_width - collision_nudge;
        let top_y = next_y;

        let left_tile = (left_x / tile_width).floor() as usize;
        let right_tile = (right_x / tile_width).floor() as usize;
        let top_tile = (top_y / tile_height).floor() as usize;

        if context.level.is_solid(left_tile, top_tile) || context.level.is_solid(right_tile, top_tile) {
            // Snap to the bottom edge of the ceiling tile and zero vertical velocity.
            pos.0.y = (top_tile as f32 * tile_height) + tile_height;
            vel.0.y = 0.0;
        } else {
            pos.0.y = next_y;
        }
    } else {
        pos.0.y = next_y;
    }

    grounded
}

/// Resolves horizontal constraints and enforces authoritative world boundaries.
pub fn resolve_horizontal_collisions(
    pos: &mut Position,
    vel: &mut Velocity,
    bounds: sdl3::rect::Rect,
    context: &SystemContext,
) -> Option<f32> {
    let tile_height = context.level.tileset.tile_height as f32;
    let tile_width = context.level.tileset.tile_width as f32;
    let scaled_bounds_width = bounds.width() as f32;
    let scaled_bounds_height = bounds.height() as f32;

    let collision_nudge = 0.2;

    // 1. Calculate the intended X position for the next tick.
    let next_x = pos.0.x + vel.0.x * context.delta_time;
    let mut wall_normal = None;

    if vel.0.x > 0.0 { 
        // 2. Resolve movement to the right.
        let top_y = pos.0.y + collision_nudge;
        let bottom_y = pos.0.y + scaled_bounds_height - collision_nudge;
        let right_x = next_x + scaled_bounds_width;

        let top_tile = (top_y / tile_height).floor() as usize;
        let bottom_tile = (bottom_y / tile_height).floor() as usize;
        let right_tile = (right_x / tile_width).floor() as usize;

        if context.level.is_solid(right_tile, top_tile) || context.level.is_solid(right_tile, bottom_tile) {
            // Stop at the wall surface and report a leftward normal.
            pos.0.x = (right_tile as f32 * tile_width) - scaled_bounds_width - collision_nudge;
            vel.0.x = 0.0;
            wall_normal = Some(-1.0);
        } else {
            pos.0.x = next_x;
        }
    } else if vel.0.x < 0.0 { 
        // 3. Resolve movement to the left.
        let top_y = pos.0.y + collision_nudge;
        let bottom_y = pos.0.y + scaled_bounds_height - collision_nudge;
        let left_x = next_x;

        let top_tile = (top_y / tile_height).floor() as usize;
        let bottom_tile = (bottom_y / tile_height).floor() as usize;
        let left_tile = (left_x / tile_width).floor() as usize;

        if context.level.is_solid(left_tile, top_tile) || context.level.is_solid(left_tile, bottom_tile) {
            // Stop at the wall surface and report a rightward normal.
            pos.0.x = (left_tile as f32 * tile_width) + tile_width + collision_nudge;
            vel.0.x = 0.0;
            wall_normal = Some(1.0);
        } else {
            pos.0.x = next_x;
        }
    } else {
        pos.0.x = next_x;
    }

    // 4. Enforce hard world boundaries defined by the level map dimensions.
    let map_width_in_tiles = context.level.map.tiles[0].len() as f32;
    let map_width = map_width_in_tiles * tile_width;

    if pos.0.x < 0.0 {
        pos.0.x = 0.0;
        vel.0.x = 0.0;
        wall_normal = Some(1.0);
    }

    if pos.0.x + scaled_bounds_width > map_width {
        pos.0.x = map_width - scaled_bounds_width;
        vel.0.x = 0.0;
        wall_normal = Some(-1.0);
    }

    wall_normal
}