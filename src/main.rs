mod app;
mod config;
mod renderer;
// mod audio;
mod texture_manager;
mod player;

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
    let _config = load_config().map_err(|e| e.to_string())?;
    let mut app = App::new().map_err(|e| e.to_string())?;
    app.run()
}
