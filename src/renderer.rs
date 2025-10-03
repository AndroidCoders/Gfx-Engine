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

/// The main rendering structure.
pub struct Renderer {}

impl Renderer {
    /// Draws the scene.
    pub fn draw(
        canvas: &mut Canvas<Window>,
        virtual_canvas_texture: &mut Texture,
        texture_manager: &TextureManager,
        player: &Player,
        level: &Level,
        camera: &Camera,
        background_color: [u8; 3],
    ) -> Result<(), String> {
        // Get the current animation frame and texture
        if let Some(texture_name) = player.animation_controller.current_texture_name() {
            if let Some(player_texture) = texture_manager.get(texture_name) {
                let src_rect = player.animation_controller.current_frame_rect().copied();

                // Set render target to virtual canvas
                canvas.with_texture_canvas(virtual_canvas_texture, |texture_canvas| {
                    texture_canvas.set_draw_color(Color::RGB(background_color[0], background_color[1], background_color[2]));
                    texture_canvas.clear();

                    // Draw level objects
                    // TODO: Move this to a config file (e.g., graphics.platform_color)
                    texture_canvas.set_draw_color(Color::RGB(255, 255, 255));
                    for object in &level.objects {
                        let rect = Rect::new(object.x - camera.x, object.y - camera.y, object.width, object.height);
                        texture_canvas.fill_rect(rect).map_err(|e| e.to_string()).unwrap();
                    }

                    // Draw player
                    let dest_rect = Rect::new((player.position.x - camera.x as f32) as i32, (player.position.y - camera.y as f32) as i32, player.width, player.height);
                    texture_canvas.copy(player_texture, src_rect.map(|r| r.into()), dest_rect).map_err(|e| e.to_string()).unwrap();
                }).map_err(|e| e.to_string())?;
            } else {
                // Texture not found, draw a fallback rectangle
                canvas.with_texture_canvas(virtual_canvas_texture, |texture_canvas| {
                    texture_canvas.set_draw_color(Color::RGB(255, 0, 255)); // Bright pink for missing texture
                    let dest_rect = Rect::new((player.position.x - camera.x as f32) as i32, (player.position.y - camera.y as f32) as i32, player.width, player.height);
                    texture_canvas.fill_rect(dest_rect).map_err(|e| e.to_string()).unwrap();
                }).map_err(|e| e.to_string())?;
            }
        } else {
            // No animation playing, do nothing or draw a fallback
        }

        // Copy virtual canvas to main canvas
        // TODO: Use the background color from the config file (config.window.background_color)
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