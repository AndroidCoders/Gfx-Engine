mod app;
mod animation;
mod config;
mod renderer;
mod state_machine;
mod audio;
mod input;
mod math;
mod physics;
mod camera;
mod level;
mod texture_manager;
mod player;
mod enemy;
mod ecs;

use app::App;

/// The main entry point of the application.
///
/// This function loads the application configuration, initializes the `App`
/// struct, and runs the main application loop.
///
/// # Returns
///
/// A `Result` indicating success (`()`) or an error (`String`).
fn main() -> Result<(), String> {
    // Initialize SDL
    let sdl_context = sdl3::init().map_err(|e| e.to_string())?;

    let mut app = App::new(sdl_context).map_err(|e| e.to_string())?;
    app.run()
}

