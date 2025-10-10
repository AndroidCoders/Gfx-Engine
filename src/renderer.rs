// src/renderer.rs

//! Handles all drawing operations for the engine.

use sdl3::pixels::Color;
use sdl3::render::{Canvas, Texture};
use sdl3::video::Window;
use sdl3::rect::Rect;

use crate::texture_manager::TextureManager;
use crate::player::Player;
use crate::camera::Camera;

use crate::level::Level;

/// A context struct holding references to data needed for rendering.
pub struct RenderContext<'a> {
    pub texture_manager: &'a TextureManager,
    pub camera: &'a Camera,
    pub background_color: [u8; 3],
}

/// The main rendering structure.
pub struct Renderer {}

impl Renderer {
    /// Draws the scene.
    pub fn draw(
        canvas: &mut Canvas<Window>,
        virtual_canvas_texture: &mut Texture,
        context: &RenderContext,
        player: &Player,
        level: &Level,
    ) -> Result<(), String> {
        canvas.with_texture_canvas(virtual_canvas_texture, |texture_canvas| {
            // Clear the canvas
            texture_canvas.set_draw_color(Color::RGB(context.background_color[0], context.background_color[1], context.background_color[2]));
            texture_canvas.clear();

            // --- Draw Static Background ---
            let bg_sky = context.texture_manager.get("bg_sky").unwrap();
            texture_canvas.copy(bg_sky, None, None).unwrap();


            // --- Draw the Visual Tilemap ---
            let tileset_texture = context.texture_manager.get(&level.tileset.texture).unwrap();
            let tileset_width_in_tiles = tileset_texture.query().width / level.tileset.tile_width;
            let scale = 2.0;
            let scaled_tile_width = (level.tileset.tile_width as f32 * scale) as u32;
            let scaled_tile_height = (level.tileset.tile_height as f32 * scale) as u32;

            for (y, row) in level.map.tiles.iter().enumerate() {
                for (x, &tile_id) in row.iter().enumerate() {
                    if tile_id == 0 { continue; }

                    let tile_id = tile_id - 1; // Adjust for 1-based indexing
                    
                    let tile_x_in_tileset = tile_id % tileset_width_in_tiles;
                    let tile_y_in_tileset = tile_id / tileset_width_in_tiles;

                    let src_x = (tile_x_in_tileset * level.tileset.tile_width) as i32;
                    let src_y = (tile_y_in_tileset * level.tileset.tile_height) as i32;
                    let src_rect = Rect::new(src_x, src_y, level.tileset.tile_width, level.tileset.tile_height);

                    let dest_x = (x as u32 * scaled_tile_width) as i32 - context.camera.position.x as i32;
                    let dest_y = (y as u32 * scaled_tile_height) as i32 - context.camera.position.y as i32;
                    let dest_rect = Rect::new(dest_x, dest_y, scaled_tile_width, scaled_tile_height);

                    texture_canvas.copy(tileset_texture, src_rect, dest_rect).unwrap();
                }
            }

            // --- Draw the Player ---
            if let Some(texture_name) = player.animation_controller.current_texture_name() {
                if let Some(player_texture) = context.texture_manager.get(texture_name) {
                    let src_rect = player.animation_controller.current_frame_rect().copied();
                    let draw_x = player.position.x - (player.draw_width - player.width) as f32 / 2.0 + player.horizontal_draw_offset as f32;
                    let draw_y = player.position.y + player.height as f32 - player.draw_height as f32 + player.vertical_draw_offset as f32;
                    let dest_rect = Rect::new((draw_x - context.camera.position.x) as i32, (draw_y - context.camera.position.y) as i32, player.draw_width, player.draw_height);
                    texture_canvas.copy(player_texture, src_rect.map(|r| r.into()), dest_rect).unwrap();
                }
            }
        }).map_err(|e| e.to_string())?;

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.copy(
            virtual_canvas_texture,
            None,
            None,
        ).map_err(|e| e.to_string())?;

        Ok(())
    }
}
