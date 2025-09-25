// src/renderer.rs

//! Handles all drawing operations for the engine.

use sdl3::pixels::Color;
use sdl3::render::{Canvas, Texture};
use sdl3::video::Window;
use sdl3::rect::Rect;



/// The main rendering structure.
pub struct Renderer {
}

impl Renderer {
    /// Creates a new stateless `Renderer` instance.
    pub fn new() -> Self {
        Self {}
    }

    /// Draws a white rectangle on a black background.
    pub fn draw(
        canvas: &mut Canvas<Window>,

        virtual_canvas_texture: &mut Texture,
        background_color: [u8; 3],
    ) -> Result<(), String> {
        // Set render target to virtual canvas
        canvas.with_texture_canvas(virtual_canvas_texture, |texture_canvas| {
            texture_canvas.set_draw_color(Color::RGB(background_color[0], background_color[1], background_color[2]));
            texture_canvas.clear();

            // Draw a white rectangle to the virtual canvas
            texture_canvas.set_draw_color(Color::RGB(255, 255, 255));
            texture_canvas.fill_rect(Rect::new(100, 100, 50, 50)).map_err(|e| e.to_string()).unwrap();
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