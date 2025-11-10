// src/renderer.rs

//! This module handles all rendering operations for the game engine.
//! 
//! It provides the `Renderer` struct, which encapsulates the SDL3 `WindowCanvas`
//! and provides methods for drawing shapes, textures, and text.
//! 
//! # Examples
//! 
//! ```no_run
//! use sdl3::pixels::Color;
//! use crate::renderer::Renderer;
//! use crate::camera::Camera;
//! use crate::texture_manager::TextureManager;
//! use crate::level::Level;
//! 
//! // Assuming you have a canvas, camera, texture_manager, and level
//! // let mut renderer = Renderer::new(canvas).unwrap();
//! // renderer.clear(Color::RGB(0, 0, 0));
//! // renderer.draw_level(&level, &texture_manager, &camera).unwrap();
//! // renderer.present();
//! ```

use sdl3::render::WindowCanvas;
use sdl3::pixels::Color;
use crate::level::Level;
use crate::camera::Camera;
use crate::math::Vector2D;
use crate::texture_manager::TextureManager;
use std::ffi::CString;
use sdl3_sys::everything::SDL_Renderer;

unsafe extern "C" {
    /// A raw C-style function for rendering simple debug text.
    /// This is a temporary solution for debugging and will be replaced by a
    /// proper text rendering system using `sdl3_ttf`.
    pub fn SDL_RenderDebugText(renderer: *mut SDL_Renderer, x: f32, y: f32, text: *const libc::c_char);
}

/// The main rendering context for the application.
///
/// This struct wraps the SDL `WindowCanvas` and provides a high-level API
/// for all drawing operations, such as clearing the screen, drawing sprites,
/// and rendering level geometry.
pub struct Renderer {
    canvas: WindowCanvas,
}

impl Renderer {
    /// Creates a new `Renderer` from an SDL `WindowCanvas`.
    pub fn new(canvas: WindowCanvas) -> Result<Self, String> {
        Ok(Self { canvas })
    }

    /// Draws debug text on the screen at a given position.
    ///
    /// Note: This is a temporary debug function and will be replaced.
    pub fn draw_debug_text(&mut self, text: &str, x: i32, y: i32) -> Result<(), String> {
        let c_text = CString::new(text).map_err(|e| e.to_string())?;
        unsafe {
            SDL_RenderDebugText(self.canvas.raw(), x as f32, y as f32, c_text.as_ptr());
        }
        Ok(())
    }

    /// Sets the current drawing color for the renderer.
    pub fn set_draw_color(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
    }

    /// Clears the entire screen with a given color.
    pub fn clear(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    /// Presents the back buffer to the screen, updating what is visible.
    pub fn present(&mut self) {
        self.canvas.present();
    }

    /// Draws the entire visible portion of a level, including the background and tiles.
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

    /// Draws a rectangle outline on the screen (primarily for debugging).
    pub fn draw_rect(&mut self, rect: &sdl3::rect::Rect, color: Color) -> Result<(), String> {
        self.canvas.set_draw_color(color);
        self.canvas.draw_rect((*rect).into()).map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Draws a sprite (a portion of a texture) on the screen.
    ///
    /// The sprite's position is specified in world coordinates and is transformed
    /// by the camera and the global `PIXEL_SCALE`.
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
