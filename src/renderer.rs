//! # Manager: Visual Engine (SDL3 Bridge)
//! 
//! This module handles all physical drawing operations. It encapsulates the 
//! SDL3 WindowCanvas and provides a high-level API for rendering the game 
//! world, UI elements, and cinematic transitions.

use sdl3::render::{WindowCanvas, FRect};
use sdl3::pixels::Color;
use crate::level::Level;
use crate::camera::Camera;
use crate::math::Vector2D;
use crate::texture_manager::TextureManager;
use crate::font_manager::FontManager;

/// The primary context for GPU-accelerated 2D rendering.
pub struct Renderer {
    pub canvas: WindowCanvas,
}

pub struct SpriteDrawParams<'a> {
    pub pos: Vector2D,
    pub size: (u32, u32),
    pub offsets: (i32, i32),
    pub texture_name: &'a str,
    pub frame_rect: &'a sdl3::rect::Rect,
    pub color_mod: Option<Color>,
    pub rotation: f64,
    pub flip_horizontal: bool,
    pub flip_vertical: bool,
}

pub struct TextRenderParams<'a> {
    pub text: &'a str,
    pub x: i32,
    pub y: i32,
    pub font_size: f32,
    pub scale: f32,
    pub color: Color,
}

impl Renderer {
    pub fn new(canvas: WindowCanvas) -> Result<Self, String> { Ok(Self { canvas }) }

    pub fn output_size(&self) -> (u32, u32) { self.canvas.output_size().unwrap() }
    #[allow(dead_code)]
    pub fn set_draw_color(&mut self, color: Color) { self.canvas.set_draw_color(color); }
    pub fn clear(&mut self, color: Color) { self.canvas.set_draw_color(color); self.canvas.clear(); }
    pub fn present(&mut self) { self.canvas.present(); }

    pub fn copy(&mut self, texture: &sdl3::render::Texture, src: Option<sdl3::rect::Rect>, dst: Option<sdl3::rect::Rect>) -> Result<(), String> {
        self.canvas.copy(
            texture,
            src.map(|r| FRect::new(r.x as f32, r.y as f32, r.width() as f32, r.height() as f32)),
            dst.map(|r| FRect::new(r.x as f32, r.y as f32, r.width() as f32, r.height() as f32))
        ).map_err(|e| e.to_string())
    }

    pub fn fill_rect(&mut self, rect: &sdl3::rect::Rect, color: Color) -> Result<(), String> {
        self.canvas.set_blend_mode(sdl3::render::BlendMode::Blend);
        self.canvas.set_draw_color(color);
        let frect = FRect::new(rect.x as f32, rect.y as f32, rect.width() as f32, rect.height() as f32);
        self.canvas.fill_rect(frect).map_err(|e| e.to_string())?;
        self.canvas.set_blend_mode(sdl3::render::BlendMode::None);
        Ok(())
    }

    pub fn draw_rect(&mut self, rect: &sdl3::rect::Rect, color: Color) -> Result<(), String> {
        self.canvas.set_draw_color(color);
        self.canvas.draw_rect((*rect).into()).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn render_text(&mut self, font_manager: &FontManager, params: TextRenderParams) -> Result<(), String> {
        let surface = font_manager.render_surface("debug", params.text, params.font_size, params.color)?;
        let texture_creator = self.canvas.texture_creator();
        let mut texture = texture_creator.create_texture_from_surface(&surface).map_err(|e| e.to_string())?;
        texture.set_blend_mode(sdl3::render::BlendMode::Blend);
        unsafe { sdl3_sys::render::SDL_SetTextureScaleMode(texture.raw(), sdl3_sys::surface::SDL_SCALEMODE_NEAREST); }
        let width = (surface.width() as f32 * params.scale) as u32;
        let height = (surface.height() as f32 * params.scale) as u32;
        let dest_rect = sdl3::rect::Rect::new(params.x, params.y, width, height);
        self.canvas.copy(&texture, None, dest_rect).map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Renders the static environment and parallax background layers.
    pub fn draw_level(&mut self, level: &Level, texture_manager: &TextureManager, camera: &Camera, parallax_config: &crate::config::ParallaxConfig) -> Result<(), String> {
        let scale = crate::config::RENDER_SCALE_FACTOR;
        let mut layers = parallax_config.layers.clone();
        layers.sort_by(|a, b| b.z_index.cmp(&a.z_index));

        for layer in layers {
            if let Some(texture) = texture_manager.get(&layer.texture) {
                let q = texture.query();
                // Calculate dimensions in Screen Space
                let scaled_width = q.width as f32 * scale;
                let scaled_height = q.height as f32 * scale;
                
                // Calculate scroll offset in Retro Space
                let scroll_x = camera.position.x * layer.scroll_speed_x;
                let scroll_y = camera.position.y * layer.scroll_speed_y;
                
                // Calculate initial draw position in Screen Space
                let mut draw_x = (-scroll_x % q.width as f32) * scale;
                if draw_x > 0.0 { draw_x -= scaled_width; }
                
                let screen_width = self.canvas.output_size().unwrap().0 as f32;
                
                while draw_x < screen_width {
                    let draw_y = -scroll_y * scale;
                    let dest_rect = sdl3::rect::Rect::new(draw_x as i32, draw_y as i32, scaled_width as u32, scaled_height as u32);
                    self.canvas.copy(texture, None, dest_rect).map_err(|e| e.to_string())?;
                    draw_x += scaled_width;
                }
            }
        }

        if let Some(tileset_texture) = texture_manager.get(&level.tileset.texture) {
            let tile_width = level.tileset.tile_width;
            let tile_height = level.tileset.tile_height;
            
            // Culling logic remains in Retro Space (using virtual_width/height which are now 480x270)
            let min_col = (camera.position.x / tile_width as f32).floor() as i32;
            let max_col = ((camera.position.x + camera.virtual_width) / tile_width as f32).ceil() as i32;
            let min_row = (camera.position.y / tile_height as f32).floor() as i32;
            let max_row = ((camera.position.y + camera.virtual_height) / tile_height as f32).ceil() as i32;
            
            let start_row = min_row.max(0) as usize;
            let end_row = (max_row as usize).min(level.map.tiles.len());
            let start_col = min_col.max(0) as usize;
            let end_col = (max_col as usize).min(level.map.tiles[0].len());

            for row_idx in start_row..end_row {
                let row = &level.map.tiles[row_idx];
                for (col_idx, &tile_id) in row.iter().enumerate().take(end_col).skip(start_col) {
                    if tile_id == 0 { continue; } 
                    let tileset_cols = tileset_texture.query().width / tile_width;
                    let src_x = ((tile_id - 1) % tileset_cols) * tile_width;
                    let src_y = ((tile_id - 1) / tileset_cols) * tile_height;
                    let src_rect = sdl3::rect::Rect::new(src_x as i32, src_y as i32, tile_width, tile_height);
                    
                    // Scale positions to Screen Space
                    let dest_x = ((col_idx as f32 * tile_width as f32) - camera.position.x) * scale;
                    let dest_y = ((row_idx as f32 * tile_height as f32) - camera.position.y) * scale;
                    let dest_rect = sdl3::rect::Rect::new(dest_x as i32, dest_y as i32, (tile_width as f32 * scale) as u32, (tile_height as f32 * scale) as u32);
                    
                    self.canvas.copy(tileset_texture, src_rect, dest_rect).map_err(|e| e.to_string())?;
                }
            }
        }
        Ok(())
    }

    pub fn draw_sprite(&mut self, params: SpriteDrawParams, texture_manager: &mut TextureManager, camera: &Camera) -> Result<(), String> {
        let scale = crate::config::RENDER_SCALE_FACTOR;
        if let Some(texture) = texture_manager.get_mut(params.texture_name) {
            let dest_rect = sdl3::rect::Rect::new(
                (((params.pos.x - camera.position.x) + params.offsets.0 as f32) * scale) as i32,
                (((params.pos.y - camera.position.y) + params.offsets.1 as f32) * scale) as i32,
                (params.size.0 as f32 * scale) as u32,
                (params.size.1 as f32 * scale) as u32,
            );
            if let Some(color) = params.color_mod { texture.set_color_mod(color.r, color.g, color.b); }
            let fsrc = FRect::new(params.frame_rect.x as f32, params.frame_rect.y as f32, params.frame_rect.width() as f32, params.frame_rect.height() as f32);
            let fdst = FRect::new(dest_rect.x as f32, dest_rect.y as f32, dest_rect.width() as f32, dest_rect.height() as f32);
            self.canvas.copy_ex(texture, fsrc, fdst, params.rotation, None, params.flip_horizontal, params.flip_vertical).map_err(|e| e.to_string())?;
            if params.color_mod.is_some() { texture.set_color_mod(255, 255, 255); }
        }
        Ok(())
    }
}