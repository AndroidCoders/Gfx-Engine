// src/app.rs

//! The core application structure and main loop.

use sdl3::EventPump;
use sdl3::video::Window;
use sdl3::render::Canvas;
use sdl3::event::Event;
// use sdl3::keyboard::Keycode;

use crate::config::{Config, load_config};
use crate::renderer::Renderer;

/// The main application struct, holding all state and context.
pub struct App {
    /// The application's configuration, loaded from `config.toml`.
    #[allow(dead_code)]
    config: Config,
    /// The SDL canvas for rendering.
    canvas: Canvas<Window>,
    /// The SDL event pump for handling user input.
    event_pump: EventPump,
    /// The renderer for drawing to the canvas.
    renderer: Renderer,
}

impl App {
    /// Creates a new `App` instance, initializing SDL and creating the window.
    pub fn new() -> Result<Self, String> {
        // Initialize SDL
        let sdl_context = sdl3::init().map_err(|e| e.to_string())?;
        let video_subsystem = sdl_context.video().map_err(|e| e.to_string())?;

        // Load configuration
        let config = load_config().map_err(|e| e.to_string())?;

        // Set VSync hint
        if config.window.vsync {
            sdl3::hint::set("SDL_RENDER_VSYNC", "1");
        } else {
            sdl3::hint::set("SDL_RENDER_VSYNC", "0");
        }

        // Create the window
        let mut window_builder = video_subsystem.window(
            &config.window.title,
            config.window.width,
            config.window.height,
        );
        if config.window.fullscreen {
            window_builder.fullscreen();
        }
        let window = window_builder.build().map_err(|e| e.to_string())?;

        // Create the canvas
        let canvas = window.into_canvas();

        // Set scaling quality hint
        sdl3::hint::set("SDL_HINT_RENDER_SCALE_QUALITY", &config.window.scaling_quality);

        // Create the event pump
        let event_pump = sdl_context.event_pump().map_err(|e| e.to_string())?;

        // Create the renderer
        let renderer = Renderer::new(config.window.background_color);

        // Return the new App instance
        Ok(Self {
            config: config.clone(),
            canvas,
            event_pump,
            renderer,
        })
    }

    /// Runs the main game loop until the user quits.
    pub fn run(&mut self) -> Result<(), String> {
        'running: loop {
            // Process events
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit {..} => break 'running,
                    Event::KeyDown { keycode: Some(_keycode), .. } => {
                        // if _keycode.name() == self.config.input.quit_key {
                            break 'running
                        // }
                    },
                    _ => {}
                }
            }

            // Draw the scene
            self.renderer.draw(&mut self.canvas)?;
            self.canvas.present();
        }
        Ok(())
    }
}
