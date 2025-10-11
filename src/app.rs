// src/app.rs

//! The core application structure and main loop.

use sdl3::EventPump;
use sdl3::video::Window;
use sdl3::render::{Canvas, Texture};

use crate::config::{Config, GameConfig, load_config, load_game_config};
use crate::renderer::{Renderer, RenderContext};
use crate::texture_manager::TextureManager;
use crate::player::Player;
use crate::input::{InputHandler, InputState};
use crate::level::{Level, load_level};
use crate::camera::Camera;

use crate::audio::GameAudioManager;

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
    audio_manager: GameAudioManager,
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
        let game_config = load_game_config("game_config.toml").map_err(|e| e.to_string())?;

        // Load level data
        let level = load_level("assets/levels/world_1_level_1.tmx")?;

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

        // Create the texture manager and load all textures
        let mut texture_manager = TextureManager::new();
        // Load animation textures
        for anim_config in game_config.animation.values() {
            texture_manager.load(&anim_config.texture, &anim_config.texture, &texture_creator)?;
        }
        // Load tileset texture from the level
        texture_manager.load(&level.tileset.texture, &level.tileset.texture, &texture_creator)?;

        // Load background textures
        texture_manager.load("assets/graphics/background_blue_sky_with_clouds.png", "bg_sky", &texture_creator)?;

        // Create the virtual canvas texture
        let virtual_canvas_texture = texture_creator
            .create_texture_target(None, config.window.virtual_width, config.window.virtual_height)
            .map_err(|e| e.to_string())?;

        // Set scaling quality hint
        sdl3::hint::set("SDL_HINT_RENDER_SCALE_QUALITY", &config.window.scaling_quality);

        // Create the event pump
        let event_pump = sdl_context.event_pump().map_err(|e| e.to_string())?;

        // Initialize Audio Manager
        let mut audio_manager = GameAudioManager::new()?;
        audio_manager.load_sound("assets/sounds/sfx_jump_01.ogg", "jump")?;

        // Create the player
        let mut player = Player::new(&game_config.player);
        // Set player start position from level entities
        if let Some(player_entity) = level.entities.iter().find(|e| e.r#type == "Player") {
            player.position.x = player_entity.x as f32;
            player.position.y = player_entity.y as f32;
        }

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

        // Create the camera
        let camera = Camera::new(player.position.x, player.position.y, config.window.camera_tightness);

        // Return the new App instance
        Ok(Self {
            config: config.clone(),
            game_config,
            canvas,
            event_pump,
            virtual_canvas_texture: Some(virtual_canvas_texture),
            texture_manager,
            audio_manager,
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
                &self.audio_manager.event_sender(),
            );

            // Process audio events
            self.audio_manager.process_events();

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
                self.player.velocity = crate::math::Vector2D::default();
            }


            // Update camera
            let target_x = self.player.position.x - (self.virtual_width / 2) as f32;
            let target_y = self.player.position.y - (self.virtual_height / 2) as f32;
            self.camera.update(crate::math::Vector2D::new(target_x, target_y));

            // Clamp camera to world boundaries
            if self.camera.position.x < 0.0 {
                self.camera.position.x = 0.0;
                self.camera.velocity.x = 0.0;
            }
            if self.camera.position.x + self.virtual_width as f32 > world_width {
                self.camera.position.x = world_width - self.virtual_width as f32;
                self.camera.velocity.x = 0.0;
            }
            if self.camera.position.y < 0.0 {
                self.camera.position.y = 0.0;
                self.camera.velocity.y = 0.0;
            }

            // *****************************************************************
            //  Draw
            // *****************************************************************
            let context = RenderContext {
                texture_manager: &self.texture_manager,
                camera: &self.camera,
                background_color: self.config.window.background_color,
            };
            Renderer::draw(
                &mut self.canvas,
                self.virtual_canvas_texture.as_mut().expect("Virtual canvas texture should be initialized"),
                &context,
                &self.player,
                &self.level,
            )?;
            self.canvas.present();
        }
        Ok(())
    }
}