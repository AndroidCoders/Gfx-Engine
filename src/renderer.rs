// src/renderer.rs

//! Handles all drawing operations for the engine.

use sdl3::pixels::Color;
use sdl3::render::{Canvas, Texture};
use sdl3::video::Window;
use sdl3::rect::Rect;
use crate::texture_manager::TextureManager;
use crate::player::Player;
use crate::level::Level;
use crate::camera::Camera;
use crate::config::GameConfig;

/// A context struct holding references to data needed for rendering.
pub struct RenderContext<'a> {
    pub texture_manager: &'a TextureManager,
    pub game_config: &'a GameConfig,
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
        // Get the current animation frame and texture
        if let Some(texture_name) = player.animation_controller.current_texture_name() {
            if let Some(player_texture) = context.texture_manager.get(texture_name) {
                let src_rect = player.animation_controller.current_frame_rect().copied();

                // Set render target to virtual canvas
                canvas.with_texture_canvas(virtual_canvas_texture, |texture_canvas| {
                    texture_canvas.set_draw_color(Color::RGB(context.background_color[0], context.background_color[1], context.background_color[2]));
                    texture_canvas.clear();

                    // --- Draw Tiled Platforms ---
                    let tileset_path = &context.game_config.graphics.platform_tileset;
                    if let Some(tileset_texture) = context.texture_manager.get(tileset_path) {
                        let tile_config = &context.game_config.graphics.platform_tile;
                        let src_tile_rect = Rect::new(
                            tile_config.x,
                            tile_config.y,
                            tile_config.width,
                            tile_config.height,
                        );

                        for object in &level.objects {
                            let platform_rect = Rect::new(object.x - context.camera.x, object.y - context.camera.y, object.width, object.height);
                            texture_canvas.set_clip_rect(Some(platform_rect));

                            let tile_w = tile_config.width as i32;
                            let tile_h = tile_config.height as i32;
                            let scaled_tile_w = tile_w * 2;
                            let scaled_tile_h = tile_h * 2;

                            // Align the tiling pattern to the top-left of the platform
                            let start_x = object.x - context.camera.x;
                            let start_y = object.y - context.camera.y;

                            let num_tiles_x = (object.width as i32 + scaled_tile_w - 1) / scaled_tile_w;
                            let num_tiles_y = (object.height as i32 + scaled_tile_h - 1) / scaled_tile_h;

                            for i in 0..num_tiles_y {
                                for j in 0..num_tiles_x {
                                    let dest_rect = Rect::new(
                                        start_x + j * scaled_tile_w,
                                        start_y + i * scaled_tile_h,
                                        scaled_tile_w as u32,
                                        scaled_tile_h as u32,
                                    );
                                    texture_canvas.copy(tileset_texture, src_tile_rect, dest_rect).map_err(|e| e.to_string()).unwrap();
                                }
                            }
                            // Reset clip rect for next draw calls
                            texture_canvas.set_clip_rect(None);
                        }
                    } else {
                        // Fallback: if texture is missing, draw white rectangles
                        texture_canvas.set_draw_color(Color::RGB(255, 255, 255));
                        for object in &level.objects {
                            let rect = Rect::new(object.x - context.camera.x, object.y - context.camera.y, object.width, object.height);
                            texture_canvas.fill_rect(rect).map_err(|e| e.to_string()).unwrap();
                        }
                    }

                    // Center the sprite horizontally over the collision box and apply offset
                    let draw_x = player.position.x - (player.draw_width - player.width) as f32 / 2.0 + player.horizontal_draw_offset as f32;
                    // Align the bottom of the sprite with the bottom of the collision box and apply offset
                    let draw_y = player.position.y + player.height as f32 - player.draw_height as f32 + player.vertical_draw_offset as f32;
                    let dest_rect = Rect::new((draw_x - context.camera.x as f32) as i32, (draw_y - context.camera.y as f32) as i32, player.draw_width, player.draw_height);
                    texture_canvas.copy(player_texture, src_rect.map(|r| r.into()), dest_rect).map_err(|e| e.to_string()).unwrap();
                }).map_err(|e| e.to_string())?;
            } else {
                // Texture not found, draw a fallback rectangle
                canvas.with_texture_canvas(virtual_canvas_texture, |texture_canvas| {
                    texture_canvas.set_draw_color(Color::RGB(255, 0, 255)); // Bright pink for missing texture
                    let draw_x = player.position.x - (player.draw_width - player.width) as f32 / 2.0 + player.horizontal_draw_offset as f32;
                    let draw_y = player.position.y + player.height as f32 - player.draw_height as f32 + player.vertical_draw_offset as f32;
                    let dest_rect = Rect::new((draw_x - context.camera.x as f32) as i32, (draw_y - context.camera.y as f32) as i32, player.draw_width, player.draw_height);
                    texture_canvas.fill_rect(dest_rect).map_err(|e| e.to_string()).unwrap();
                }).map_err(|e| e.to_string())?;
            }
        } else {
            // No animation playing, do nothing or draw a fallback
        }

        // Copy virtual canvas to main canvas
        canvas.set_draw_color(Color::RGB(0, 0, 0)); // Clear main canvas
        canvas.clear();
        canvas.copy(
            virtual_canvas_texture,
            None,
            None,
        ).map_err(|e| e.to_string())?;

        Ok(())
    }
}