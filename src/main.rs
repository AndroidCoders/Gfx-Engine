mod app;
mod game_state;
mod renderer;
mod config;
mod texture_manager;
mod physics;

mod frame_capture;

use app::App;
use config::load_config;

/// The main entry point of the application.
///
/// This function loads the application configuration, initializes the `App`
/// struct, and runs the main application loop.
///
/// # Returns
///
/// A `Result` indicating success (`()`) or an error (`String`).
fn main() -> Result<(), String> {
    let config = load_config().map_err(|e| e.to_string())?;
    let mut app = App::new(&config).map_err(|e| e.to_string())?;
    app.run()
}