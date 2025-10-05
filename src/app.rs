// src/app.rs

//! The core application structure and main loop.

use sdl3::EventPump;
use sdl3::video::Window;
use sdl3::render::{Canvas, Texture};

use crate::config::{Config, GameConfig, load_config, load_game_config};
use crate::renderer::Renderer;
use crate::texture_manager::TextureManager;
use crate::player::Player;
use crate::input::{InputHandler, InputState};
use crate::level::{Level, load_level};
use crate::camera::Camera;

/// The main application struct, holding all state and context.
pub struct App {
    /// The application's configuration, loaded from `config.toml`.
    #[allow(dead_code)]
    config: Config,
    game_config: GameConfig,
    /// The SDL canvas for rendering.
    canvas: Canvas<Window>,
    /// The SDL event pump for handling user input.
    event_pump: EventPump,
    /// The virtual canvas texture.
    virtual_canvas_texture: Option<Texture>,
    texture_manager: TextureManager,
    level: Level,
    camera: Camera,
    player: Player,
    input_handler: InputHandler,
    input_state: InputState,
    /// The virtual width of the game canvas.
    virtual_width: u32,
    /// The virtual height of the game canvas.
    #[allow(dead_code)]
    virtual_height: u32,
}

impl App {
    /// Creates a new `App` instance, initializing SDL and creating the window.
    pub fn new() -> Result<Self, String> {
        // Initialize SDL
        let sdl_context = sdl3::init().map_err(|e| e.to_string())?;
        let video_subsystem = sdl_context.video().map_err(|e| e.to_string())?;

        // Load configuration
        let config = load_config().map_err(|e| e.to_string())?;
        // TODO: Move this to a config file
        let game_config = load_game_config("game_config.toml").map_err(|e| e.to_string())?;

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

        // Create the canvas and texture creator
        let canvas = window.into_canvas();
        let texture_creator = canvas.texture_creator();

        // Create the texture manager and load all animation textures
        let mut texture_manager = TextureManager::new();
        let mut unique_textures = std::collections::HashSet::new();
        for anim_config in game_config.animation.values() {
            unique_textures.insert(anim_config.texture.clone());
        }
        for texture_path in unique_textures {
            texture_manager.load(&texture_path, &texture_path, &texture_creator)?;
        }
        // Create the virtual canvas texture
        let virtual_canvas_texture = texture_creator
            .create_texture_target(None, config.window.virtual_width, config.window.virtual_height)
            .map_err(|e| e.to_string())?;

        // Set scaling quality hint
        sdl3::hint::set("SDL_HINT_RENDER_SCALE_QUALITY", &config.window.scaling_quality);

        // Create the event pump
        let event_pump = sdl_context.event_pump().map_err(|e| e.to_string())?;

        // Create the player
        let mut player = Player::new(&game_config.player);

        // Load animations
        for (name, anim_config) in &game_config.animation {
            let mut frames = Vec::new();
            for i in 0..anim_config.frame_count {
                frames.push(sdl3::rect::Rect::new(
                    anim_config.start_x + (i * anim_config.frame_width) as i32,
                    anim_config.start_y,
                    anim_config.frame_width,
                    anim_config.frame_height,
                ));
            }
            let animation = crate::animation::Animation {
                texture_name: anim_config.texture.clone(),
                frames,
                frame_duration: anim_config.frame_duration,
                loops: anim_config.loops,
            };
            player.animation_controller.add_animation(name.clone(), animation);
        }

        // Create input handler and state
        let input_handler = InputHandler::new(config.input.clone());
        let input_state = InputState::default();

        // Load level data
        let level = load_level(&game_config.assets.level)?;

        // Create the camera
        let camera = Camera::new(0, 0);

        // Return the new App instance
        Ok(Self {
            config: config.clone(),
            game_config,
            canvas,
            event_pump,
            virtual_canvas_texture: Some(virtual_canvas_texture),
            texture_manager,
            level,
            camera,
            player,
            input_handler,
            input_state,
            virtual_width: config.window.virtual_width,
            virtual_height: config.window.virtual_height,
        })
    }

    /// Runs the main game loop until the user quits.
    pub fn run(&mut self) -> Result<(), String> {
        'running: loop {
            // Process events
            if !self.input_handler.process_events(&mut self.event_pump, &mut self.input_state) {
                break 'running;
            }

            // *****************************************************************
            //  Update State
            // *****************************************************************
            crate::player::state::update_player_state(
                &mut self.player,
                &self.input_state,
                &self.config,
                &self.level,
            );

            // --- World Boundary and Camera ---

            // Clamp player position to world boundaries
            let world_width = self.game_config.world.width;
            if self.player.position.x < 0.0 {
                self.player.position.x = 0.0;
            }
            if self.player.position.x + self.player.width as f32 > world_width {
                self.player.position.x = world_width - self.player.width as f32;
            }
            // Add a death plane
            if self.player.position.y > self.game_config.world.death_plane_y {
                self.player.position = crate::math::Vector2D::new(self.game_config.player.respawn_x, self.game_config.player.respawn_y);
            }


            // Update camera
            self.camera.x = (self.player.position.x - (self.virtual_width / 2) as f32) as i32;

            // Clamp camera to world boundaries
            if self.camera.x < 0 {
                self.camera.x = 0;
            }
            if self.camera.x + self.virtual_width as i32 > world_width as i32 {
                self.camera.x = (world_width - self.virtual_width as f32) as i32;
            }

            // *****************************************************************
            //  Draw
            // *****************************************************************
            Renderer::draw(
                &mut self.canvas,
                self.virtual_canvas_texture.as_mut().expect("Virtual canvas texture should be initialized"),
                &self.texture_manager,
                &self.player,
                &self.level,
                &self.camera,
                self.config.window.background_color,
            )?;
            self.canvas.present();
        }
        Ok(())
    }
}
