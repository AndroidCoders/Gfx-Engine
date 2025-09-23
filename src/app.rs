// src/app.rs

//! The core application structure and main loop.

use sdl3::EventPump;
use sdl3::video::Window;
use sdl3::render::Canvas;
use sdl3::event::Event;
// use sdl3::keyboard::Keycode;

use crate::config::{Config, load_config};
use crate::renderer::Renderer;

/// The main application struct.
pub struct App {
    #[allow(dead_code)]
    config: Config,
    canvas: Canvas<Window>,
    event_pump: EventPump,
    renderer: Renderer,
}

impl App {
    /// Creates a new `App` instance.
    pub fn new() -> Result<Self, String> {
        let sdl_context = sdl3::init().map_err(|e| e.to_string())?;
        let video_subsystem = sdl_context.video().map_err(|e| e.to_string())?;

        let config = load_config().map_err(|e| e.to_string())?;

        if config.window.vsync {
            sdl3::hint::set("SDL_RENDER_VSYNC", "1");
        } else {
            sdl3::hint::set("SDL_RENDER_VSYNC", "0");
        }

        let mut window_builder = video_subsystem.window(
            &config.window.title,
            config.window.width,
            config.window.height,
        );
        if config.window.fullscreen {
            window_builder.fullscreen();
        }
        let window = window_builder.build().map_err(|e| e.to_string())?;

        let canvas = window.into_canvas();

        sdl3::hint::set("SDL_HINT_RENDER_SCALE_QUALITY", &config.window.scaling_quality);

        let event_pump = sdl_context.event_pump().map_err(|e| e.to_string())?;

        let renderer = Renderer::new(config.window.background_color);

        Ok(Self {
            config: config.clone(),
            canvas,
            event_pump,
            renderer,
        })
    }

    /// Runs the main game loop.
    pub fn run(&mut self) -> Result<(), String> {
        'running: loop {
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

            self.renderer.draw(&mut self.canvas)?;
            self.canvas.present();
        }
        Ok(())
    }
}
