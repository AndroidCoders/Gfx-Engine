// src/renderer.rs

//! Handles all drawing operations for the engine.

use sdl3::render::WindowCanvas;
use sdl3::pixels::Color;
use crate::level::Level;
use crate::camera::Camera;
use crate::math::Vector2D;
use crate::texture_manager::TextureManager;
use std::ffi::CString;
use sdl3_sys::everything::SDL_Renderer;

unsafe extern "C" {
    pub fn SDL_RenderDebugText(renderer: *mut SDL_Renderer, x: f32, y: f32, text: *const libc::c_char);
}

pub struct Renderer {
    canvas: WindowCanvas,
}

impl Renderer {
    pub fn new(canvas: WindowCanvas) -> Result<Self, String> {
        Ok(Self { canvas })
    }

    pub fn draw_debug_text(&mut self, text: &str, x: i32, y: i32) -> Result<(), String> {
        let c_text = CString::new(text).map_err(|e| e.to_string())?;
        unsafe {
            SDL_RenderDebugText(self.canvas.raw(), x as f32, y as f32, c_text.as_ptr());
        }
        Ok(())
    }

    pub fn set_draw_color(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
    }

    pub fn clear(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn draw_level(&mut self, level: &Level, texture_manager: &TextureManager, camera: &Camera) -> Result<(), String> {
        // Draw background
        if let Some(bg_texture) = texture_manager.get("bg_sky") {
            self.canvas.copy(bg_texture, None, None).map_err(|e| e.to_string())?;
        }

        // Draw tiles
        if let Some(tileset_texture) = texture_manager.get(&level.tileset.texture) {
            let tile_width = level.tileset.tile_width;
            let tile_height = level.tileset.tile_height;

            for (row_idx, row) in level.map.tiles.iter().enumerate() {
                for (col_idx, &tile_id) in row.iter().enumerate() {
                    if tile_id == 0 { continue; } // Skip empty tiles

                    // Calculate source rectangle in the tileset texture
                    let tileset_cols = tileset_texture.query().width / tile_width;
                    let src_x = ((tile_id - 1) % tileset_cols) * tile_width;
                    let src_y = ((tile_id - 1) / tileset_cols) * tile_height;
                    let src_rect = sdl3::rect::Rect::new(src_x as i32, src_y as i32, tile_width, tile_height);

                    // Calculate destination rectangle on the screen
                    let dest_x = ((col_idx as f32 * tile_width as f32) - camera.position.x) * crate::config::PIXEL_SCALE;
                    let dest_y = ((row_idx as f32 * tile_height as f32) - camera.position.y) * crate::config::PIXEL_SCALE;
                    let dest_rect = sdl3::rect::Rect::new(dest_x as i32, dest_y as i32, (tile_width as f32 * crate::config::PIXEL_SCALE) as u32, (tile_height as f32 * crate::config::PIXEL_SCALE) as u32);

                    self.canvas.copy(tileset_texture, src_rect, dest_rect).map_err(|e| e.to_string())?;
                }
            }
        }
        Ok(())
    }

    pub fn draw_sprite(&mut self, pos: Vector2D, size: (u32, u32), offsets: (i32, i32), texture_name: &str, frame_rect: &sdl3::rect::Rect, texture_manager: &TextureManager, camera: &Camera) -> Result<(), String> {
        if let Some(texture) = texture_manager.get(texture_name) {
            let dest_rect = sdl3::rect::Rect::new(
                (((pos.x - camera.position.x) + offsets.0 as f32) * crate::config::PIXEL_SCALE) as i32,
                (((pos.y - camera.position.y) + offsets.1 as f32) * crate::config::PIXEL_SCALE) as i32,
                (size.0 as f32 * crate::config::PIXEL_SCALE) as u32,
                (size.1 as f32 * crate::config::PIXEL_SCALE) as u32,
            );
            self.canvas.copy(texture, *frame_rect, dest_rect).map_err(|e| e.to_string())?;
        }
        Ok(())
    }
}
