use sdl3::EventPump;
use sdl3::Sdl;

use crate::config::{Config, GameConfig, load_config, load_game_config};
use crate::texture_manager::TextureManager;
use crate::input::{InputHandler, InputState};
use crate::level::{Level, load_level};
use crate::camera::Camera;
use crate::audio::GameAudioManager;
use crate::ecs::{
    world::{World, Entity},
    system::*,
    component::*,
};
use crate::renderer::Renderer;
use crate::math::Vector2D;
use crate::state_machine::StateMachine;
use crate::player::states::IdleState;

use crate::animation::AnimationController;

/// The main application struct, holding all state and context.
pub struct App {
    /// The application's configuration, loaded from `config.toml`.
    #[allow(dead_code)]
    config: Config,
    _game_config: GameConfig,
    /// The virtual width of the game canvas.
    _virtual_width: u32,
    renderer: Renderer,
    event_pump: EventPump,
    texture_manager: TextureManager,
    audio_manager: GameAudioManager,
    level: Level,
    camera: Camera,
    input_handler: InputHandler,
    input_state: InputState,
    world: World,
    player_entity: Option<Entity>,
    frame_count: u64,
    _sdl_context: Sdl,
    /// The virtual height of the game canvas.
    #[allow(dead_code)]
    _virtual_height: u32,
}

impl App {
    pub fn new(sdl_context: Sdl) -> Result<App, String> {

                // Load configuration



                let config = load_config().map_err(|e| e.to_string())?;



                let game_config = load_game_config("game_config.toml").map_err(|e| e.to_string())?;

        

                // Load level data



                let level = load_level("assets/levels/world_1_level_1.tmx")?;

        

                let video_subsystem = sdl_context.video().map_err(|e| e.to_string())?;

        

                // Set rendering hints BEFORE creating the renderer

                sdl3::hint::set("SDL_RENDER_SCALE_QUALITY", &config.window.scaling_quality);

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
        let texture_creator = canvas.texture_creator();

        // Create the texture manager and load all textures
        let mut texture_manager = TextureManager::new();

        for anim_config in game_config.animation.values() {
            texture_manager.load(&anim_config.texture, &anim_config.texture, &texture_creator)?;
        }

        texture_manager.load(&level.tileset.texture, &level.tileset.texture, &texture_creator)?;

        texture_manager.load("assets/graphics/background_blue_sky_with_clouds.png", "bg_sky", &texture_creator)?;

        // Create the world and systems
        let mut world = World::new();

        // Create the player entity and determine its starting position from config
        let player_entity = world.create_entity();
        let player_position = Position(Vector2D::new(game_config.player.start_x, game_config.player.start_y));

        // Create the camera using world units
        let map_width_in_tiles = level.map.tiles[0].len() as f32;
        let map_height_in_tiles = level.map.tiles.len() as f32;
        let total_map_width = map_width_in_tiles * level.tileset.tile_width as f32;
        let total_map_height = map_height_in_tiles * level.tileset.tile_height as f32;

        let virtual_width_in_world = config.window.virtual_width as f32 / crate::config::PIXEL_SCALE;
        let virtual_height_in_world = config.window.virtual_height as f32 / crate::config::PIXEL_SCALE;

        // Calculate the center of the player sprite
        let player_center_x = player_position.0.x + (game_config.player.width as f32 / 2.0);
        let player_center_y = player_position.0.y + (game_config.player.height as f32 / 2.0);

        // Calculate the desired top-left corner of the camera in world units
        let initial_camera_x = player_center_x - (virtual_width_in_world / 2.0);
        let initial_camera_y = player_center_y - (virtual_height_in_world / 2.0);

        // Clamp initial camera position to map boundaries
        let initial_camera_x_clamped = initial_camera_x.clamp(0.0, total_map_width - virtual_width_in_world);
        let initial_camera_y_clamped = initial_camera_y.clamp(0.0, total_map_height - virtual_height_in_world);

        let camera = Camera::new(
            initial_camera_x_clamped,
            initial_camera_y_clamped,
            config.window.camera_tightness,
            virtual_width_in_world,
            virtual_height_in_world,
            total_map_width,
            total_map_height,
        );

        // Create the renderer
        let renderer = Renderer::new(canvas)?;

        // Create the event pump
        let event_pump = sdl_context.event_pump().map_err(|e| e.to_string())?;

        // Initialize Audio Manager
        let mut audio_manager = GameAudioManager::new()?;

        audio_manager.load_sound("assets/sounds/sfx_jump_01.ogg", "jump")?;
        world.add_position(player_entity, player_position);
        world.add_velocity(player_entity, Velocity(Vector2D::default()));
        world.add_renderable(player_entity, Renderable {
            width: game_config.player.draw_width,
            height: game_config.player.draw_height,
            horizontal_offset: game_config.player.horizontal_draw_offset,
            vertical_offset: game_config.player.vertical_draw_offset,
        });

        let mut animation_controller = AnimationController::new();
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
            animation_controller.add_animation(name.clone(), animation);
        }
        world.add_animation(player_entity, Animation { controller: animation_controller });
        world.add_player_tag(player_entity, PlayerTag);
        world.add_gravity(player_entity, Gravity);
        world.add_collision(player_entity, Collision {
            rect: sdl3::rect::Rect::new(
                player_position.0.x as i32,
                player_position.0.y as i32,
                game_config.player.width,
                game_config.player.height,
            ),
        });
        world.add_state_component(player_entity, StateComponent { state_machine: StateMachine::new(IdleState) });


        // Create input handler and state
        let input_handler = InputHandler::new(config.input.clone());
        let input_state = InputState::default();

        // Return the new App instance
        Ok(Self {
            config: config.clone(),
            _game_config: game_config,
            renderer,
            event_pump,
            texture_manager,
            audio_manager,
            level,
            camera,
            input_handler,
            input_state,
            world,
            player_entity: Some(player_entity),
            frame_count: 0,
            _sdl_context: sdl_context,
            _virtual_width: config.window.virtual_width,
            _virtual_height: config.window.virtual_height,
        })
    }

    /// Runs the main game loop until the user quits.
    pub fn run(&mut self) -> Result<(), String> {
        'running: loop {
            self.frame_count += 1;
            // Process events
            if !self.input_handler.process_events(&mut self.event_pump, &mut self.input_state) {
                break 'running;
            }

            // --- Create system instances locally ---
            let mut input_system = InputSystem;
            let mut physics_system = PhysicsSystem;
            let mut collision_system = CollisionSystem;
            let mut animation_system = AnimationSystem;
            let mut state_machine_system = StateMachineSystem;
            let mut audio_system = AudioSystem;
            let mut death_system = DeathSystem;
            let mut respawn_system = RespawnSystem;
            let mut respawn_timer_system = RespawnTimerSystem;

            // --- Create a mutable context for systems ---
            let mut system_context = SystemContext {
                level: &self.level,
                _camera: &self.camera,
                input_state: &self.input_state,
                config: &self.config,
                game_config: &self._game_config,
                audio_sender: &self.audio_manager.event_sender(),
            };

            // --- Run systems ---
            input_system.update(&mut self.world, &mut system_context);
            physics_system.update(&mut self.world, &mut system_context);
            collision_system.update(&mut self.world, &mut system_context);
            death_system.update(&mut self.world, &mut system_context);
            respawn_system.update(&mut self.world, &mut system_context);
            respawn_timer_system.update(&mut self.world, &mut system_context);
            state_machine_system.update(&mut self.world, &mut system_context);
            animation_system.update(&mut self.world, &mut system_context);
            audio_system.update(&mut self.world, &mut self.audio_manager);

            // --- Rendering ---
            self.renderer.clear(sdl3::pixels::Color::RGB(0, 0, 0));
            self.renderer.draw_level(&self.level, &self.texture_manager, &self.camera)?;

            if let Some(player_pos) = self.world.positions.values().next() {
                let player_renderable = self.world.renderables.values().next().unwrap();
                if let Some(player_animation) = self.world.animations.get(&self.player_entity.unwrap()) {
                    if let (Some(player_texture_name), Some(player_frame_rect)) = (
                        player_animation.controller.current_texture_name(),
                        player_animation.controller.current_frame_rect(),
                    ) {
                        self.renderer.draw_player(player_pos.0, (player_renderable.width, player_renderable.height), (player_renderable.horizontal_offset, player_renderable.vertical_offset), player_texture_name, player_frame_rect, &self.texture_manager, &self.camera)?;
                    }
                }
            }

            // --- Update camera ---
            if let Some(player_entity) = self.player_entity {
                if let (Some(player_pos), Some(renderable)) = (
                    self.world.positions.get(&player_entity),
                    self.world.renderables.get(&player_entity),
                ) {
                    let sprite_center_x = player_pos.0.x + renderable.horizontal_offset as f32 + (renderable.width as f32 / 2.0);
                    let sprite_center_y = player_pos.0.y + renderable.vertical_offset as f32 + (renderable.height as f32 / 2.0);
                    self.camera.update(Vector2D::new(sprite_center_x, sprite_center_y));
                }
            }

            // --- Debug Output ---
            if let Some(player_entity) = self.player_entity {
                if let (Some(pos), Some(vel), Some(state_comp), Some(collision)) = (
                    self.world.positions.get(&player_entity),
                    self.world.velocities.get(&player_entity),
                    self.world.state_components.get(&player_entity),
                    self.world.collisions.get(&player_entity),
                ) {
                    let is_grounded = self.world.is_grounded(player_entity);
                    let state_name = state_comp.state_machine.current_state.as_ref().map_or("None", |s| s.get_name());

                    let debug_text_color = sdl3::pixels::Color::RGB(255, 255, 255);
                    self.renderer.set_draw_color(debug_text_color);
                    self.renderer.draw_debug_text(&format!("Frame: {}", self.frame_count), 10, 10)?;
                    self.renderer.draw_debug_text(&format!("Player Pos: ({:.2}, {:.2})", pos.0.x, pos.0.y), 10, 30)?;
                    self.renderer.draw_debug_text(&format!("Vel: ({:.2}, {:.2})", vel.0.x, vel.0.y), 10, 50)?;
                    self.renderer.draw_debug_text(&format!("State: {}", state_name), 10, 70)?;
                    self.renderer.draw_debug_text(&format!("Grounded: {}", is_grounded), 10, 90)?;
                    self.renderer.draw_debug_text(&format!("Collision Rect: {:?}", collision.rect), 10, 110)?;
                }
            }

            // --- Present the frame ---
            self.renderer.present();
        }
        Ok(())
    }
}
