use sdl3::pixels::Color;
use sdl3::render::Canvas;
use sdl3::video::Window;
use crate::game_state::GameState;
use crate::config::RendererConfig;

pub struct Renderer {
    background_color: Color,
    object_color: Color,
}

impl Renderer {
    pub fn new(config: &RendererConfig) -> Self {
        let bg = config.background_color;
        let obj = config.object_color;
        Self {
            background_color: Color::RGB(bg[0], bg[1], bg[2]),
            object_color: Color::RGB(obj[0], obj[1], obj[2]),
        }
    }

    /// Draws the current game state onto the canvas.
    ///
    /// # Arguments
    ///
    /// * `canvas` - A mutable reference to the SDL `Canvas`.
    /// * `game_state` - A reference to the current `GameState`.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success (`()`) or an error (`String`).
pub fn draw(&self, canvas: &mut Canvas<Window>, game_state: &GameState) -> Result<(), sdl3::render::TargetRenderError> {
        canvas.set_draw_color(self.background_color);
        canvas.clear();
        canvas.set_draw_color(self.object_color);
        for object in &game_state.objects {
            canvas.fill_rect(object.to_rect()).map_err(|e| sdl3::render::TargetRenderError::SdlError(e))?;
        }
        // No canvas.present() here, as we are drawing to a texture
        Ok(())
    }
}
