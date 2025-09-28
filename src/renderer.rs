// src/renderer.rs

//! Handles all drawing operations for the engine.

use sdl3::pixels::Color;
use sdl3::render::{Canvas, Texture};
use sdl3::video::Window;
use sdl3::rect::Rect;
use crate::texture_manager::TextureManager;
use crate::player::{Player, PlayerDirection};
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
        // Determine which texture to use
        let player_texture_name = match player.direction {
            PlayerDirection::Front => "player_front",
            PlayerDirection::Left => "player_left",
            PlayerDirection::Right => "player_right",
        };
        let player_texture = texture_manager.get(player_texture_name).unwrap();

        // Set render target to virtual canvas
        canvas.with_texture_canvas(virtual_canvas_texture, |texture_canvas| {
            texture_canvas.set_draw_color(Color::RGB(background_color[0], background_color[1], background_color[2]));
            texture_canvas.clear();

            // Draw level objects
            texture_canvas.set_draw_color(Color::RGB(255, 255, 255));
            for object in &level.objects {
                let rect = Rect::new(object.x - camera.x, object.y - camera.y, object.width, object.height);
                texture_canvas.fill_rect(rect).map_err(|e| e.to_string()).unwrap();
            }

            // Draw player
            let query = player_texture.query();
            texture_canvas.copy(player_texture, None, Rect::new((player.position.x - camera.x as f32) as i32, (player.position.y - camera.y as f32) as i32, query.width, query.height)).map_err(|e| e.to_string()).unwrap();
        }).map_err(|e| e.to_string())?;

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