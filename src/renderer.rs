// src/renderer.rs

//! Handles all drawing operations for the engine.

use sdl3::pixels::Color;
use sdl3::render::Canvas;
use sdl3::video::Window;
use sdl3::rect::Rect;
// use crate::config::GameConfig;

/// The main rendering structure.
pub struct Renderer {
    background_color: Color,
}

impl Renderer {
    /// Creates a new `Renderer`.
    pub fn new(background_color: [u8; 3]) -> Self {
        Self {
            background_color: Color::RGB(background_color[0], background_color[1], background_color[2]),
        }
    }

    /// Draws a white rectangle on a black background.
    pub fn draw(
        &self,
        canvas: &mut Canvas<Window>,
        // game_config: &GameConfig,
    ) -> Result<(), String> {
        canvas.set_draw_color(self.background_color);
        canvas.clear();

        // Draw a white rectangle
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.fill_rect(Rect::new(100, 100, 50, 50)).map_err(|e| e.to_string())?;

        Ok(())
    }
}