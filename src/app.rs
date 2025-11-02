use crate::ecs::systems::System;
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
    systems::{self,
        animation_update::AnimationUpdateSystem,
        audio::AudioSystem,
        coin_collection::CoinCollectionSystem,
        death::DeathSystem,
        input::InputSystem,
        interaction::InteractionSystem,
        kill::KillSystem,
        physics::PhysicsSystem,
        player_animation::PlayerAnimationSystem,
        respawn::RespawnSystem,
        respawn_timer::RespawnTimerSystem,
        state_machine::StateMachineSystem,
        tile_collision::TileCollisionSystem,
        invincibility::InvincibilitySystem,
        player_death::PlayerDeathSystem,
        lifetime::LifetimeSystem,
    },
    component::*,
};
use crate::renderer::Renderer;
use crate::math::Vector2D;
use crate::state_machine::StateMachine;
use crate::player::states::IdleState;
use crate::enemy::states::PatrolState;


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
    show_debug_info: bool,
    gold_coin_count: u32,
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

                                                let mut texture_manager = TextureManager::new();

                                                for (_name, anim_config) in &game_config.animation {
                                                    texture_manager.load(&anim_config.texture, &anim_config.texture, &texture_creator)?;
                                                }

                                                texture_manager.load(&level.tileset.texture, &level.tileset.texture, &texture_creator)?;

                                                texture_manager.load("assets/graphics/background_blue_sky_with_clouds.png", "bg_sky", &texture_creator)?;

                                        

                                                // Create the world and systems
                                                let mut world = World::new();
                                                let mut player_entity = None;

                                                // Create entities from level data
                                                for entity_data in &level.entities {
                                                    let entity = world.create_entity();
                                                    match entity_data.r#type.as_str() {
                                                        "GoldCoin" => {
                                                            let coin_config = &game_config.collectible["gold_coin"];
                                                            world.add_position(entity, Position(entity_data.position));
                                                            world.add_renderable(entity, Renderable {
                                                                width: coin_config.draw_width,
                                                                height: coin_config.draw_height,
                                                                horizontal_offset: 0,
                                                                vertical_offset: 0,
                                                                z_index: 100,
                                                            });
                                                            let mut coin_animation_controller = AnimationController::new();
                                                            if let Some(anim_config) = game_config.animation.get("gold_coin_spin") {
                                                                let mut frames = Vec::new();
                                                                for i in 0..anim_config.frame_count {
                                                                    let padding = anim_config.frame_padding.unwrap_or(0);
                                                                    frames.push(sdl3::rect::Rect::new(
                                                                        anim_config.start_x + (i * (anim_config.frame_width + padding)) as i32,
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
                                                                coin_animation_controller.add_animation("gold_coin_spin".to_string(), animation);
                                                                coin_animation_controller.set_animation("gold_coin_spin");
                                                            }
                                                            world.add_animation(entity, Animation { controller: coin_animation_controller });
                                                            world.add_gold_coin(entity, GoldCoin);
                                                            world.add_collision(entity, Collision {
                                                                rect: sdl3::rect::Rect::new(
                                                                    entity_data.position.x as i32,
                                                                    entity_data.position.y as i32,
                                                                    coin_config.width,
                                                                    coin_config.height,
                                                                ),
                                                            });
                                                        }
                                                        "EnemySpider" => {
                                                            let enemy_spider_pos = Position(entity_data.position);
                                                            let enemy_spider_config = &game_config.enemy["enemy_spider"];
                                                            world.add_position(entity, enemy_spider_pos);
                                                            world.add_velocity(entity, Velocity(Vector2D::new(enemy_spider_config.speed, 0.0)));
                                                            world.add_renderable(entity, Renderable {
                                                                width: enemy_spider_config.draw_width,
                                                                height: enemy_spider_config.draw_height,
                                                                horizontal_offset: 0,
                                                                vertical_offset: 0,
                                                                z_index: 100,
                                                            });
                                                            let mut enemy_spider_animation_controller = AnimationController::new();
                                                            for (name, anim_config) in &game_config.animation {
                                                                if name.starts_with("enemy_spider") {
                                                                    let mut frames = Vec::new();
                                                                    for i in 0..anim_config.frame_count {
                                                                        let padding = anim_config.frame_padding.unwrap_or(0);
                                                                        frames.push(sdl3::rect::Rect::new(
                                                                            anim_config.start_x + (i * (anim_config.frame_width + padding)) as i32,
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
                                                                    enemy_spider_animation_controller.add_animation(name.clone(), animation);
                                                                }
                                                            }
                                                            enemy_spider_animation_controller.set_animation("enemy_spider_1");
                                                            world.add_animation(entity, Animation { controller: enemy_spider_animation_controller });
                                                            world.add_enemy_tag(entity, EnemyTag);
                                                            world.add_patrol(entity, Patrol { speed: enemy_spider_config.speed });
                                                            world.add_gravity(entity, Gravity);
                                                            world.add_collision(entity, Collision {
                                                                rect: sdl3::rect::Rect::new(
                                                                    enemy_spider_pos.0.x as i32,
                                                                    enemy_spider_pos.0.y as i32,
                                                                    enemy_spider_config.width,
                                                                    enemy_spider_config.height,
                                                                ),
                                                            });
                                                            world.add_state_component(entity, StateComponent { state_machine: StateMachine::new(PatrolState) });
                                                        }
                                                        _ => {}
                                                    }
                                                }

                                                // Create the player entity
                                                let player_entity_instance = world.create_entity();
                                                let player_position = Position(game_config.player.start_pos);
                                                world.add_position(player_entity_instance, player_position);
                                                world.add_velocity(player_entity_instance, Velocity(Vector2D::default()));
                                                world.add_renderable(player_entity_instance, Renderable {
                                                    width: game_config.player.draw_width,
                                                    height: game_config.player.draw_height,
                                                    horizontal_offset: game_config.player.horizontal_draw_offset,
                                                    vertical_offset: game_config.player.vertical_draw_offset,
                                                    z_index: 100,
                                                });
                                                let mut player_animation_controller = AnimationController::new();
                                                for (name, anim_config) in &game_config.animation {
                                                    if !name.starts_with("enemy_spider") && !name.starts_with("gold_coin") {
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
                                                        player_animation_controller.add_animation(name.clone(), animation);
                                                    }
                                                }
                                                world.add_animation(player_entity_instance, Animation { controller: player_animation_controller });
                                                world.add_player_tag(player_entity_instance, PlayerTag);
                                                world.add_gravity(player_entity_instance, Gravity);
                                                world.add_collision(player_entity_instance, Collision {
                                                    rect: sdl3::rect::Rect::new(
                                                        player_position.0.x as i32,
                                                        player_position.0.y as i32,
                                                        game_config.player.width,
                                                        game_config.player.height,
                                                    ),
                                                });
                                                world.add_state_component(player_entity_instance, StateComponent { state_machine: StateMachine::new(IdleState) });
                                                world.add_health(player_entity_instance, Health { current: 3, max: 3 });
                                                player_entity = Some(player_entity_instance);

                                                // The camera creation needs the player's starting position.
                                                let player_start_pos = game_config.player.start_pos;

                                                // Create the camera using world units
                                                let map_width_in_tiles = level.map.tiles[0].len() as f32;
                                                let map_height_in_tiles = level.map.tiles.len() as f32;
                                                let total_map_width = map_width_in_tiles * level.tileset.tile_width as f32;
                                                let total_map_height = map_height_in_tiles * level.tileset.tile_height as f32;

                                                let virtual_width_in_world = config.window.virtual_width as f32 / crate::config::PIXEL_SCALE;
                                                let virtual_height_in_world = config.window.virtual_height as f32 / crate::config::PIXEL_SCALE;

                                                // Calculate the center of the player sprite
                                                let player_center_x = player_start_pos.x + (game_config.player.width as f32 / 2.0);
                                                let player_center_y = player_start_pos.y + (game_config.player.height as f32 / 2.0);

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
                                                    config.window.camera_slow_zone,
                                                    config.window.camera_fast_zone,
                                                    config.window.camera_vertical_snap_threshold,
                                                    config.window.camera_vertical_tightness,
                                                    config.window.camera_falling_tightness,
                                                    config.window.camera_falling_velocity_threshold,
                                                    config.physics.entity_max_fall_speed,
                                                );

                                                // Create the renderer
                                                let renderer = Renderer::new(canvas)?;

                                                // Create the event pump
                                                let event_pump = sdl_context.event_pump().map_err(|e| e.to_string())?;

                                                // Initialize Audio Manager
                                                let audio_manager = GameAudioManager::new(&game_config.audio)?;

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
            player_entity,
            frame_count: 0,
            _sdl_context: sdl_context,
            _virtual_width: config.window.virtual_width,
            _virtual_height: config.window.virtual_height,
            show_debug_info: true,
            gold_coin_count: 0,
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

            if self.input_state.is_debug_action_just_pressed(crate::input::DebugAction::ToggleDebugInfo) {
                self.show_debug_info = !self.show_debug_info;
            }

            // --- Create system instances locally ---
            let mut input_system = InputSystem;
            let mut physics_system = PhysicsSystem;
            let mut tile_collision_system = TileCollisionSystem;
            let mut interaction_system = InteractionSystem;
            let mut coin_collection_system = CoinCollectionSystem;
            let mut kill_system = KillSystem;
            let mut player_animation_system = PlayerAnimationSystem;
            let mut animation_update_system = AnimationUpdateSystem;
            let mut state_machine_system = StateMachineSystem;
            let mut audio_system = AudioSystem;
            let mut death_system = DeathSystem;
            let mut respawn_system = RespawnSystem;
            let mut respawn_timer_system = RespawnTimerSystem;
            let mut invincibility_system = InvincibilitySystem;
            let mut player_death_system = PlayerDeathSystem;
            let mut lifetime_system = LifetimeSystem;

            // --- Create a mutable context for systems ---
            let mut system_context = systems::SystemContext {
                level: &self.level,
                input_state: &self.input_state,
                config: &self.config,
                game_config: &self._game_config,
                audio_sender: &self.audio_manager.event_sender(),
                gold_coin_count: &mut self.gold_coin_count,
            };

            // --- Run systems ---
            input_system.update(&mut self.world, &mut system_context);
            physics_system.update(&mut self.world, &mut system_context);
            interaction_system.update(&mut self.world, &mut system_context);
            tile_collision_system.update(&mut self.world, &mut system_context);
            player_death_system.update(&mut self.world, &mut system_context);
            coin_collection_system.update(&mut self.world, &mut system_context);
            kill_system.update(&mut self.world, &mut system_context);
            death_system.update(&mut self.world, &mut system_context);
            let mut respawn_system_context = systems::RespawnSystemContext {
                camera: &mut self.camera,
                game_config: &self._game_config,
            };
            respawn_system.update(&mut self.world, &mut respawn_system_context);
            respawn_timer_system.update(&mut self.world, &mut system_context);
            invincibility_system.update(&mut self.world, &mut system_context);
            lifetime_system.update(&mut self.world, &mut system_context);
            state_machine_system.update(&mut self.world, &mut system_context);
            player_animation_system.update(&mut self.world, &mut system_context);
            animation_update_system.update(&mut self.world, &mut system_context);
            audio_system.update(&mut self.world, &mut self.audio_manager);

            // --- Rendering ---
            self.renderer.clear(sdl3::pixels::Color::RGB(0, 0, 0));
            self.renderer.draw_level(&self.level, &self.texture_manager, &self.camera)?;

            // Collect all renderable entities into a list
            let mut renderables_sorted: Vec<(u8, Entity)> = Vec::new();
            for (entity, renderable) in &self.world.renderables {
                // Ensure the entity also has a position before adding it to the render list
                if self.world.positions.contains_key(entity) {
                    renderables_sorted.push((renderable.z_index, *entity));
                }
            }

            // Sort the list by z_index (higher z_index is further back, rendered first)
            renderables_sorted.sort_by_key(|k| k.0);

            // Draw the sorted entities
            for (_, entity) in renderables_sorted {
                if let Some(pos) = self.world.positions.get(&entity) {
                    if let Some(renderable) = self.world.renderables.get(&entity) {
                        if let Some(animation) = self.world.animations.get(&entity) {
                            if let (Some(texture_name), Some(frame_rect)) = (
                                animation.controller.current_texture_name(),
                                animation.controller.current_frame_rect(),
                            ) {
                                self.renderer.draw_sprite(pos.0, (renderable.width, renderable.height), (renderable.horizontal_offset, renderable.vertical_offset), texture_name, frame_rect, &self.texture_manager, &self.camera)?;
                            }
                        }
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
                    let is_grounded = self.world.is_grounded(player_entity);
                    let player_vel_y = self.world.velocities.get(&player_entity).map_or(0.0, |v| v.0.y);
                    self.camera.update(Vector2D::new(sprite_center_x, sprite_center_y), is_grounded, player_vel_y);
                }
            }

            // --- Debug Output ---
            if self.show_debug_info {
                if self.config.debug.debug_draw_collision_boxes {
                    for (_entity, collision) in &self.world.collisions {
                        let rect = sdl3::rect::Rect::new(
                            ((collision.rect.x as f32 - self.camera.position.x) * crate::config::PIXEL_SCALE) as i32,
                            ((collision.rect.y as f32 - self.camera.position.y) * crate::config::PIXEL_SCALE) as i32,
                            (collision.rect.width() as f32 * crate::config::PIXEL_SCALE) as u32,
                            (collision.rect.height() as f32 * crate::config::PIXEL_SCALE) as u32,
                        );
                        self.renderer.draw_rect(&rect, sdl3::pixels::Color::RGB(255, 0, 0))?;
                    }
                }

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
                        self.renderer.draw_debug_text(&format!("Gold: {}", self.gold_coin_count), 10, 130)?;
                    }
                }
            }

            // --- Present the frame ---
            self.renderer.present();
        }
        Ok(())
    }
}
