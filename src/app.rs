use sdl3::EventPump;
use sdl3::video::Window;
use sdl3::render::Canvas;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;

use crate::config::Config;
use crate::game_state::GameState;
use crate::renderer::Renderer;
use crate::frame_capture::FrameCapture;

use crate::texture_manager::TextureManager;

pub struct App<'a> {
    config: Config,
    canvas: Canvas<Window>,
    game_state: GameState,
    event_pump: EventPump,
    renderer: Option<Renderer>,
    timestep: f64,
    texture_manager: TextureManager<'a>,
    frame_capture: Option<FrameCapture>,
}

impl<'a> App<'a> {
    pub fn new(config: &Config) -> Result<Self, Box<dyn std::error::Error>> {
        let sdl_context = sdl3::init()?;
        let video_subsystem = sdl_context.video()?;

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

        let window = window_builder.build()?;

        let canvas = window.into_canvas();
        let texture_creator = canvas.texture_creator();
        let texture_manager = TextureManager::new(texture_creator, config.window.virtual_width, config.window.virtual_height)?;

        sdl3::hint::set("SDL_HINT_RENDER_SCALE_QUALITY", &config.window.scaling_quality);

        let game_state = GameState::new(&config.objects);
        let event_pump = sdl_context.event_pump()?;
        let timestep = 1.0 / config.physics.updates_per_second as f64;
        let frame_capture = if config.debug.enable_frame_capture {
            Some(FrameCapture::new(&config.debug))
        } else {
            None
        };

        let mut app = Self {
            config: config.clone(),
            canvas,
            game_state,
            event_pump,
            renderer: None,
            timestep,
            texture_manager,
            frame_capture,
        };

        let renderer = Renderer::new(&config.renderer);

        app.renderer = Some(renderer);

        Ok(app)
    }

    pub fn run(&mut self) -> Result<(), String> {
        let mut accumulator = 0.0;

        let mut last_time = sdl3::timer::performance_counter();
        let perf_freq = sdl3::timer::performance_frequency() as f64;

        'running: loop {
            let current_time = sdl3::timer::performance_counter();
            let delta_time = (current_time - last_time) as f64 / perf_freq;
            last_time = current_time;

            accumulator += delta_time;

            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {}
                }
            }

            let (width, height) = self.canvas.output_size().map_err(|e| e.to_string())?;

            while accumulator >= self.timestep {
                self.game_state.update(self.config.window.virtual_width, self.config.window.virtual_height, &self.config.physics);
                accumulator -= self.timestep;
            }

            // Draw to the virtual canvas texture
            self.canvas.with_texture_canvas(self.texture_manager.virtual_canvas_texture_mut(), |texture_canvas| {
                // The draw method returns Result<(), TargetRenderError>.
                // We need to handle the error here, as the closure must return ().
                if let Err(e) = self.renderer.as_mut().expect("Renderer should be initialized").draw(texture_canvas, &self.game_state) {
                    eprintln!("Rendering error: {}", e);
                }
            }).map_err(|e| e.to_string())?;

            // Copy the virtual canvas texture to the main canvas
            self.canvas.copy(self.texture_manager.virtual_canvas_texture(), None, None).map_err(|e| e.to_string())?;

            // Present the main canvas
            self.canvas.present();

            if let Some(fc) = &mut self.frame_capture {
                fc.capture_frame(self.game_state.frame_counter, width, height, &mut self.canvas)?;
            }
        }
        Ok(())
    }
}

impl<'a> Drop for App<'a> {
    /// Cleans up resources when the `App` instance is dropped.
    ///
    /// This includes saving any captured frames if the frame capture feature
    /// is enabled.
    fn drop(&mut self) {
        if let Some(fc) = &self.frame_capture {
            fc.save_frames();
        }
    }
}
