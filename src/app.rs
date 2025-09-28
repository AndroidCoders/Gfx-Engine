// src/app.rs

//! The core application structure and main loop.

use sdl3::EventPump;
use sdl3::video::Window;
use sdl3::render::{Canvas, Texture};
use sdl3::rect::Rect;

use crate::config::{Config, GameConfig, load_config, load_game_config};
use crate::renderer::Renderer;
use crate::texture_manager::TextureManager;
use crate::player::{Player, PlayerDirection};
use crate::input::{InputHandler, InputState, PlayerAction};
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

        // Create the texture manager and load textures
        let mut texture_manager = TextureManager::new();
        texture_manager.load(&game_config.assets.player_front, "player_front", &texture_creator)?;
        texture_manager.load(&game_config.assets.player_left, "player_left", &texture_creator)?;
        texture_manager.load(&game_config.assets.player_right, "player_right", &texture_creator)?;

        // Create the virtual canvas texture
        let virtual_canvas_texture = texture_creator
            .create_texture_target(None, config.window.virtual_width, config.window.virtual_height)
            .map_err(|e| e.to_string())?;

        // Set scaling quality hint
        sdl3::hint::set("SDL_HINT_RENDER_SCALE_QUALITY", &config.window.scaling_quality);

        // Create the event pump
        let event_pump = sdl_context.event_pump().map_err(|e| e.to_string())?;

        // Create the player
        let player = Player::new(&game_config.player);

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

            // --- Horizontal Movement ---
            let mut desired_velocity_x = 0.0;
            if self.input_state.is_action_active(PlayerAction::MoveLeft) {
                desired_velocity_x -= self.config.physics.move_speed;
                self.player.direction = PlayerDirection::Left;
            }
            if self.input_state.is_action_active(PlayerAction::MoveRight) {
                desired_velocity_x += self.config.physics.move_speed;
                self.player.direction = PlayerDirection::Right;
            }
            self.player.velocity.x = desired_velocity_x;

            // --- Vertical Movement (Jumping) ---
            if self.input_state.is_action_just_pressed(PlayerAction::Jump) && self.player.is_on_ground {
                self.player.velocity.y = self.config.physics.jump_strength;
            }

            // --- Apply Gravity ---
            self.player.velocity.y += self.config.physics.gravity;

            // --- Collision Detection (Separate Axes) ---

            // Reset on_ground flag, it will be set to true if a vertical collision occurs
            self.player.is_on_ground = false;

            // Move horizontally
            self.player.position.x += self.player.velocity.x;

            // Check for horizontal collisions
            for object in &self.level.objects {
                let player_rect = Rect::new(self.player.position.x as i32, self.player.position.y as i32, self.player.width, self.player.height);
                let object_rect = Rect::new(object.x, object.y, object.width, object.height);
                if player_rect.has_intersection(object_rect) {
                    if self.player.velocity.x > 0.0 { // Moving right
                        self.player.position.x = (object.x - self.player.width as i32) as f32;
                    } else if self.player.velocity.x < 0.0 { // Moving left
                        self.player.position.x = (object.x + object.width as i32) as f32;
                    }
                    self.player.velocity.x = 0.0;
                }
            }

            // Move vertically
            self.player.position.y += self.player.velocity.y;

            // Check for vertical collisions
            for object in &self.level.objects {
                let player_rect = Rect::new(self.player.position.x as i32, self.player.position.y as i32, self.player.width, self.player.height);
                let object_rect = Rect::new(object.x, object.y, object.width, object.height);
                if player_rect.has_intersection(object_rect) {
                    if self.player.velocity.y > 0.0 { // Moving down
                        self.player.position.y = (object.y - self.player.height as i32) as f32;
                        self.player.is_on_ground = true;
                    } else if self.player.velocity.y < 0.0 { // Moving up
                        self.player.position.y = (object.y + object.height as i32) as f32;
                    }
                    self.player.velocity.y = 0.0;
                }
            }

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
