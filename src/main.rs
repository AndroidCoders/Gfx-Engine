//! # Manager: Application Entry Point
//! 
//! This module is the root of the engine binary. It is responsible for the high-level 
//! initialization of the SDL hardware abstraction layer and the execution 
//! of the primary application loop via the `Gfx_Engine` library.

use gfx_engine::app::App;

/// Initializes the hardware context and enters the main application loop.
fn main() -> Result<(), String> {
    // 1. Initialize the SDL core and video subsystems to gain hardware access.
    let sdl_context = sdl3::init().map_err(|e| e.to_string())?;
    let _video_subsystem = sdl_context.video().map_err(|e| e.to_string())?;

    // 2. Instantiate the main App controller, which loads configs and assets.
    let mut app = App::new(sdl_context).map_err(|e| e.to_string())?;
    
    // 3. Enter the persistent run loop until a quit signal is received.
    app.run()
}
