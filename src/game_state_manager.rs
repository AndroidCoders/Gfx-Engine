//! # Manager: Application State Orchestration
//! 
//! This module is the "Grand Conductor" of the engine's high-level flow. 
//! It is responsible for level loading, world-swapping during transitions, 
//! persistent state management (stats/lives), and the coordination of 
//! the menu and replay systems.

use crate::ecs::world::{World, Entity};
use crate::ecs::system_manager::SystemManager;
use crate::level::{Level, load_level};
use crate::camera::Camera;
use crate::config::{Config, GameConfig};
use crate::player::factory::PlayerFactory;
use crate::ecs::systems::{SystemContext, RenderContext};
use crate::audio::GameAudioManager;
use crate::input::InputState;
use crate::texture_manager::TextureManager;
use crate::font_manager::FontManager;
use crate::ecs::component::Direction;
use crate::math::Vector2D;
use crate::menu::{MenuState, MenuAction};
use crate::ecs::systems::menu::SystemMenu;
use crate::ecs::resources::Screen;
use crate::replay::Replay;
use crate::benchmarker::Benchmarker;
use crate::ecs::resources::GameState;
use sdl3::render::TextureCreator;
use sdl3::video::WindowContext;

#[derive(PartialEq, Clone, Copy)]
#[allow(dead_code)]
pub enum ReplayMode { None, Recording, Playback }

/// The primary manager for orchestrating the application's high-level state and data.
pub struct GameStateManager {
    pub world: World,
    pub system_manager: SystemManager,
    pub font_manager: FontManager,
    pub benchmarker: Benchmarker,
    pub level: Level,
    pub camera: Camera,
    pub player_entity: Option<Entity>,
    pub game_over_timer: f32,
    pub next_level: Option<String>,
    pub current_level_path: String,
    pub menu_state: MenuState,
    pub menu_system: SystemMenu,
    pub selected_character_index: usize,
    pub replay_mode: ReplayMode,
    pub current_replay: Replay,
    pub replay_tick: u64,
    pub previous_replay_input: InputState,
    pub _session_benchmarks: Vec<(String, Benchmarker)>,
    /// Whether the game is currently in the process of returning to the main menu.
    pub is_exiting_to_menu: bool,
}

impl GameStateManager {
    pub fn new(config: &Config, game_config: &GameConfig, texture_manager: &mut TextureManager, texture_creator: &TextureCreator<WindowContext>) -> Result<Self, String> {
        let mut font_manager = FontManager::new();
        font_manager.load("debug", "assets/fonts/PressStart2P-Regular.ttf")?;
        let level = load_level(&config.game.start_level)?;
        texture_manager.load(&level.tileset.texture, &level.tileset.texture, texture_creator)?;
        let mut world = World::new();
        let player_entity = Some(PlayerFactory::create(&mut world, game_config));
        let camera = Self::create_camera(config, game_config, &level);
        let menu_state = MenuState::new(&game_config.menu);
        world.game_state = GameState::Menu(Screen::Main);
        world.stats.lives = game_config.player.lives;
        let (replay_mode, current_replay) = if let Ok(replay) = Replay::load("attract_mode") { (ReplayMode::Playback, replay) } else { (ReplayMode::None, Replay::default()) };
        let mut instance = Self {
            world, level, camera, system_manager: SystemManager::new(), font_manager,
            player_entity, game_over_timer: 0.0, next_level: None,
            current_level_path: config.game.start_level.clone(), menu_state,
            menu_system: SystemMenu, selected_character_index: 0, replay_mode,
            current_replay, replay_tick: 0, previous_replay_input: InputState::default(),
            benchmarker: Benchmarker::new(), _session_benchmarks: Vec::new(),
            is_exiting_to_menu: false,
        };
        instance.spawn_entities_from_level(game_config);
        Ok(instance)
    }

    fn create_camera(config: &Config, game_config: &GameConfig, level: &Level) -> Camera {
        let player_start_pos = game_config.player.start_pos;
        let map_width_in_tiles = level.map.tiles[0].len() as f32;
        let map_height_in_tiles = level.map.tiles.len() as f32;
        let total_map_width = map_width_in_tiles * level.tileset.tile_width as f32;
        let total_map_height = map_height_in_tiles * level.tileset.tile_height as f32;
        let virtual_width_in_world = config.window.virtual_width as f32;
        let virtual_height_in_world = config.window.virtual_height as f32;
        let player_center_x = player_start_pos.x + (game_config.player.width as f32 / 2.0);
        let player_center_y = player_start_pos.y + (game_config.player.height as f32 / 2.0);
        let initial_camera_x = player_center_x - (virtual_width_in_world / 2.0);
        let initial_camera_y = player_center_y - (virtual_height_in_world / 2.0);
        Camera::new(initial_camera_x, initial_camera_y, config.window.camera_tightness, virtual_width_in_world, virtual_height_in_world, total_map_width, total_map_height, config.window.camera_slow_zone, config.window.camera_fast_zone, config.window.camera_vertical_snap_threshold, config.window.camera_vertical_tightness, config.window.camera_falling_tightness, config.window.camera_falling_velocity_threshold, config.physics.entity_max_fall_speed, config.window.camera_lookahead_distance, config.window.camera_smoothing_speed)
    }

    fn spawn_entities_from_level(&mut self, game_config: &GameConfig) { crate::entity_spawner::spawn_entities(&mut self.world, &self.level, game_config); }

    fn start_game(&mut self, config: &Config, game_config: &GameConfig, audio_manager: &mut GameAudioManager) {
        self.world.game_state = GameState::Playing;
        self.replay_mode = ReplayMode::None;
        self.next_level = Some(config.game.start_level.clone());
        self.benchmarker.reset();
        self.world.stats.lives = game_config.player.lives;
        let soundtrack = self.get_soundtrack_name();
        let _ = audio_manager.event_sender().send(crate::audio::AudioEvent::StopMusic);
        self.play_soundtrack(audio_manager, &soundtrack);
    }

    fn get_soundtrack_name(&self) -> String {
        match self.selected_character_index {
            0 => "soundtrack_01".to_string(),
            1 => "soundtrack_02".to_string(),
            2 => "soundtrack_03".to_string(),
            _ => "soundtrack_01".to_string(),
        }
    }

    fn play_soundtrack(&self, audio_manager: &mut GameAudioManager, name: &str) {
        let _ = audio_manager.event_sender().send(crate::audio::AudioEvent::PlayMusic(name.to_string(), crate::audio::PlaySoundParams::default()));
    }

    #[allow(clippy::too_many_arguments)]
    pub fn update(&mut self, delta_time: f32, config: &Config, game_config: &GameConfig, input_state: &InputState, audio_manager: &mut GameAudioManager, texture_manager: &mut TextureManager, texture_creator: &TextureCreator<WindowContext>) -> Result<(), String> {
        if self.world.game_state == GameState::GameOver {
            self.game_over_timer -= delta_time;
            if self.game_over_timer <= 0.0 { self.world.game_state = GameState::Menu(Screen::Main); self.next_level = Some(config.game.start_level.clone()); self.play_soundtrack(audio_manager, "soundtrack_01"); }
            return Ok(());
        }
        // 2. Resolve the effective input fact (Hardware vs Replay Buffer).
        let mut simulated_input = InputState::default();
        let effective_input = match self.replay_mode {
            ReplayMode::Recording => { self.current_replay.frames.push(crate::replay::InputFrame { tick: self.replay_tick, pressed_actions: input_state.get_pressed_actions() }); self.replay_tick += 1; input_state },
            ReplayMode::Playback => { if let Some(frame) = self.current_replay.frames.get(self.replay_tick as usize) { simulated_input = InputState::from_actions(frame.pressed_actions.clone()); simulated_input.calculate_deltas(&self.previous_replay_input); self.previous_replay_input = InputState::from_actions(frame.pressed_actions.clone()); } else { self.replay_tick = 0; self.previous_replay_input = InputState::default(); } self.replay_tick += 1; &simulated_input },
            ReplayMode::None => input_state,
        };
        
        // Handle menu exit completion
        if self.is_exiting_to_menu && self.world.transition_finished {
            println!("[GameFlow] Transition to menu complete.");
            self.is_exiting_to_menu = false;
            self.world.transition_finished = false;
            self.world.game_state = GameState::Menu(Screen::Main);
            self.menu_state.load_screen("main");
            
            // Restart Attract Mode (Replay)
            if let Ok(replay) = Replay::load("attract_mode") {
                println!("[GameFlow] Restarting Attract Mode.");
                self.replay_mode = ReplayMode::Playback;
                self.current_replay = replay;
                self.replay_tick = 0;
                self.previous_replay_input = InputState::default();
                
                // Reset World State for Replay
                // Note: ideally we should reload the level to be clean, 
                // but for now we just respawn the player at start.
                if let Some(pe) = self.player_entity {
                    if let Some(pos) = self.world.positions.get_mut(&pe) { pos.0 = game_config.player.start_pos; }
                    if let Some(vel) = self.world.velocities.get_mut(&pe) { vel.0 = crate::math::Vector2D::default(); }
                    self.camera.snap_to(game_config.player.start_pos);
                }
            }

            // Trigger IrisIn to show the menu
            use crate::ecs::event::{EventStartTransition, TransitionType};
            self.world.event_bus.publish(EventStartTransition {
                transition_type: TransitionType::IrisIn,
                duration: 1.0,
                center: None,
            });

            self.play_soundtrack(audio_manager, "soundtrack_01");
            return Ok(());
        }

        let soundtrack_name = self.get_soundtrack_name(); // Fix borrow checker issue
        let mut system_context = SystemContext { 
            level: &self.level, 
            input_state: effective_input, 
            config, 
            game_config, 
            audio_sender: &audio_manager.event_sender(), 
            next_level: &mut self.next_level, 
            delta_time, 
            camera: &mut self.camera, 
            benchmarker: &mut self.benchmarker, 
            current_soundtrack: Some(soundtrack_name),
            is_paused: self.is_exiting_to_menu,
            is_attract_mode: self.replay_mode == ReplayMode::Playback,
        };
        
        self.system_manager.update(&mut self.world, &mut system_context, audio_manager);
        if self.world.event_bus.read::<crate::ecs::event::EventGameOver>().count() > 0 { self.world.game_state = GameState::GameOver; self.game_over_timer = game_config.gameplay.game_over_duration; let _ = audio_manager.event_sender().send(crate::audio::AudioEvent::StopMusic); }
        self.world.clear_events();
        if let Some(next_level) = self.next_level.clone() {
            self.level = load_level(&next_level)?; self.current_level_path = next_level;
            texture_manager.load(&self.level.tileset.texture, &self.level.tileset.texture, texture_creator)?;
            let previous_state = self.world.game_state.clone();
            let previous_stats = self.world.stats.clone();
            self.world = World::new(); self.world.game_state = previous_state; self.world.stats = previous_stats;
            self.next_level = None; self.spawn_entities_from_level(game_config);
            self.player_entity = Some(PlayerFactory::create(&mut self.world, game_config));
            if let Some(pe) = self.player_entity && let Some(p) = self.world.positions.get(&pe) { self.camera.snap_to(p.0); }
            use crate::ecs::event::{EventStartTransition, TransitionType};
            self.world.event_bus.publish(EventStartTransition { transition_type: TransitionType::IrisIn, duration: 1.0, center: None });
            self.play_soundtrack(audio_manager, &self.get_soundtrack_name());
        }
        if !self.is_exiting_to_menu && let Some(pe) = self.player_entity
            && let (Some(pos), Some(rend)) = (self.world.positions.get(&pe), self.world.renderables.get(&pe)) {
                let sx = pos.0.x + rend.horizontal_offset as f32 + (rend.width as f32 / 2.0);
                let sy = pos.0.y + rend.vertical_offset as f32 + (rend.height as f32 / 2.0);
                let vy = self.world.velocities.get(&pe).map_or(0.0, |v| v.0.y);
                let dir = self.world.directions.get(&pe).map_or(Direction::Right, |d| d.direction);
                self.camera.update(Vector2D::new(sx, sy), self.world.is_grounded(pe), vy, dir);
            }
        
        // --- Populate Frame Debug Info ---
        self.world.frame_debug_info.camera_pos = Some(self.camera.position);
        self.world.frame_debug_info.renderable_count = self.world.renderables.len();
        if let Some(pe) = self.player_entity {
            self.world.frame_debug_info.player_pos = self.world.positions.get(&pe).map(|p| p.0);
            self.world.frame_debug_info.player_prev_pos = self.world.previous_positions.get(&pe).map(|p| p.0);
            if let Some(r) = self.world.renderables.get(&pe) {
                self.world.frame_debug_info.player_render_w = r.width;
                self.world.frame_debug_info.player_render_h = r.height;
            }
        }
        // --- End Populate ---
        
        Ok(())
    }

    pub fn handle_input(&mut self, input_state: &InputState, audio_manager: &mut GameAudioManager, config: &Config, game_config: &GameConfig) -> Result<(), String> {
        if input_state.is_action_just_pressed(crate::input::InputAction::Quit) {
            match &self.world.game_state {
                GameState::Menu(_) => return Err("QUIT".to_string()),
                GameState::Playing => { 
                    println!("[GameFlow] Starting return to menu sequence...");
                    self.is_exiting_to_menu = true;
                    
                    // Trigger visual fade out
                    use crate::ecs::event::{EventStartTransition, TransitionType};
                    self.world.event_bus.publish(EventStartTransition {
                        transition_type: TransitionType::IrisOut,
                        duration: 1.0,
                        center: None,
                    });

                    // Trigger audio fade out
                    let _ = audio_manager.event_sender().send(crate::audio::AudioEvent::FadeOutMusic(1.0));
                    return Ok(()); 
                },
                _ => {}
            }
        }
        if let GameState::Menu(_) = self.world.game_state 
            && let Some(action) = self.menu_system.update(&mut self.menu_state, input_state) {
                match action {
                    MenuAction::StartGame => self.start_game(config, game_config, audio_manager),
                    MenuAction::Quit => return Err("QUIT".to_string()),
                    MenuAction::Navigate(target) => self.menu_state.load_screen(&target),
                    MenuAction::SelectCharacter(idx) => {
                        self.selected_character_index = idx;
                        self.start_game(config, game_config, audio_manager);
                    },
                    _ => {},
                }
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn draw(&mut self, renderer: &mut crate::renderer::Renderer, texture_manager: &TextureManager, config: &Config, game_config: &GameConfig, _input_state: &InputState, frame_count: u64, fps: u32, show_debug_info: bool, _alpha: f32) -> Result<(), String> {
        if let GameState::Menu(_) = self.world.game_state { self.menu_system.draw(renderer, &self.menu_state, &self.font_manager)?; return Ok(()); }
        let render_context = RenderContext { config, game_config, player_entity: self.player_entity, benchmarker: &self.benchmarker };
        if self.world.game_state == GameState::GameOver { self.system_manager.gui_render_system.render_game_over(renderer, texture_manager, &render_context)?; return Ok(()); }
        self.system_manager.gui_render_system.update(renderer, texture_manager, &self.font_manager, &self.world, &render_context)?;
        if show_debug_info { self.system_manager.debug_render_system.update(renderer, &self.world, &render_context, &self.camera, &self.font_manager, frame_count, fps)?; }
        self.system_manager.transition_system.draw(renderer, &render_context)?;
        Ok(())
    }
}
