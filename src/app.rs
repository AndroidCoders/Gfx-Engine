//! # Manager: Application Life-Cycle
//! 
//! This module is the engine's primary orchestrator. it is responsible for 
//! the physical initialization of the SDL2 hardware context, the management 
//! of the high-level application loop, and the temporal decoupling of 
//! deterministic logic (120Hz) from variable-rate rendering.

use crate::config::{Config, GameConfig, load_config, load_game_config};
use crate::texture_manager::TextureManager;
use crate::input::{InputHandler, InputState};
use crate::audio::GameAudioManager;
use crate::game_state_manager::GameStateManager;
use crate::ecs::world::Entity;
use crate::renderer::Renderer;
use crate::ecs::resources::GameState;
use sdl3::EventPump;
use sdl3::Sdl;

/// The root application controller holding the persistent hardware and engine state.
pub struct App {
    config: Config,
    _game_config: GameConfig,
    _virtual_width: u32,
    renderer: Renderer,
    event_pump: EventPump,
    texture_manager: TextureManager,
    audio_manager: GameAudioManager,
    input_handler: InputHandler,
    input_state: InputState,
    frame_count: u64,
    _sdl_context: Sdl,
    _virtual_height: u32,
    show_debug_info: bool,
    fps: u32,
    last_frame_time: std::time::Instant,
    fps_last_update: std::time::Instant,
    frame_count_for_fps: u32,
    game_state_manager: GameStateManager,
}

impl App {
    /// Performs physical hardware initialization and pre-loads global assets.
    pub fn new(sdl_context: Sdl) -> Result<App, String> {
        // 1. Load engine and game-specific configurations from the project root.
        let config = load_config().map_err(|e| e.to_string())?;
        let game_config = load_game_config("assets/game_config.toml").map_err(|e| e.to_string())?;

        let video_subsystem = sdl_context.video().map_err(|e| e.to_string())?;
        let mouse = sdl_context.mouse();
        mouse.show_cursor(false);

        // 2. Set hardware-level rendering hints (scaling quality, VSync) before canvas creation.
        sdl3::hint::set("SDL_RENDER_SCALE_QUALITY", &config.window.scaling_quality);
        sdl3::hint::set("SDL_RENDER_VSYNC", if config.window.vsync { "1" } else { "0" });

        // 3. Construct the OS Window and the primary GPU Canvas.
        let mut window_builder = video_subsystem.window(&config.window.title, config.window.width, config.window.height);
        if config.window.fullscreen { window_builder.fullscreen(); }
        let window = window_builder.build().map_err(|e| e.to_string())?;

        let canvas = window.into_canvas();
        let texture_creator = canvas.texture_creator();
        let mut texture_manager = TextureManager::new();

        // 4. Perform bulk loading of visual assets defined in the TOML registry.
        for anim_config in game_config.animation.values() {
            texture_manager.load(&anim_config.texture, &anim_config.texture, &texture_creator)?;
        }
        for (name, path) in &game_config.textures {
            texture_manager.load(path, name, &texture_creator)?;
        }

        // 5. Initialize high-level managers (State, Audio, Input).
        let game_state_manager = GameStateManager::new(&config, &game_config, &mut texture_manager, &texture_creator)?;
        let renderer = Renderer::new(canvas)?;
        let event_pump = sdl_context.event_pump().map_err(|e| e.to_string())?;
        let audio_manager = GameAudioManager::new(&game_config)?;

        // 6. Trigger the starting soundtrack and return the initialized controller.
        let _ = audio_manager.event_sender().send(crate::audio::AudioEvent::PlayMusic("soundtrack_01".to_string(), crate::audio::PlaySoundParams::default()));
        let input_handler = InputHandler::new(config.input.clone());

        Ok(Self {
            config: config.clone(), _game_config: game_config.clone(), renderer, event_pump,
            texture_manager, audio_manager, input_handler, input_state: InputState::default(),
            frame_count: 0, _sdl_context: sdl_context, _virtual_width: config.window.virtual_width,
            _virtual_height: config.window.virtual_height, show_debug_info: config.debug.show_debug_info,
            fps: 0, last_frame_time: std::time::Instant::now(), fps_last_update: std::time::Instant::now(),
            frame_count_for_fps: 0, game_state_manager,
        })
    }

    /// Executes the persistent run loop using a Fixed Timestep Accumulator.
    pub fn run(&mut self) -> Result<(), String> {
        const FIXED_TIMESTEP: f32 = 1.0 / 120.0;
        let mut accumulator: f32 = 0.0;

        'running: loop {
            // 1. Calculate temporal delta and update frame-rate statistics.
            let now = std::time::Instant::now();
            let mut frame_time = now.duration_since(self.last_frame_time).as_secs_f32();
            self.last_frame_time = now;
            if frame_time > 0.25 { frame_time = 0.25; }
            accumulator += frame_time;

            self.frame_count_for_fps += 1;
            if now.duration_since(self.fps_last_update).as_secs() >= 1 {
                self.fps = self.frame_count_for_fps;
                self.frame_count_for_fps = 0;
                self.fps_last_update = now;
                if self.game_state_manager.world.game_state == GameState::Playing { self.game_state_manager.benchmarker.update_fps(self.fps); }
            }

            // 2. Resolve hardware input and process debug/replay toggle facts.
            self.game_state_manager.benchmarker.push("Input");
            if !self.input_handler.process_events(&mut self.event_pump, &mut self.input_state) { break 'running; }
            if self.input_state.is_debug_action_just_pressed(crate::input::DebugAction::ToggleDebugInfo) { self.show_debug_info = !self.show_debug_info; }

            // 3. Process high-level menu navigation once per frame.
            if let Err(e) = self.game_state_manager.handle_input(&self.input_state, &mut self.audio_manager, &self.config, &self._game_config) {
                if e == "QUIT" { break 'running; } else { return Err(e); }
            }
            self.game_state_manager.benchmarker.pop();

            // 4. Advance deterministic simulation steps (Logic/Physics) at a strict 120Hz.
            self.game_state_manager.benchmarker.push("Update");
            let texture_creator = self.renderer.canvas.texture_creator();
            while accumulator >= FIXED_TIMESTEP {
                self.game_state_manager.update(FIXED_TIMESTEP, &self.config, &self._game_config, &self.input_state, &mut self.audio_manager, &mut self.texture_manager, &texture_creator)?;
                accumulator -= FIXED_TIMESTEP;
            }
            self.game_state_manager.benchmarker.pop();

            // 5. Execute variable-rate rendering pass with alpha interpolation.
            let alpha = accumulator / FIXED_TIMESTEP;
            self.game_state_manager.benchmarker.push("Render.Clear");
            self.renderer.clear(sdl3::pixels::Color::RGB(50, 50, 50));
            self.game_state_manager.benchmarker.pop();

            self.game_state_manager.benchmarker.push("Render.Level");
            self.renderer.draw_level(&self.game_state_manager.level, &self.texture_manager, &self.game_state_manager.camera, &self._game_config.parallax)?;
            self.game_state_manager.benchmarker.pop();

            self.game_state_manager.benchmarker.push("Render.Entities");
            let visible_entities = self.game_state_manager.world.spatial_grid.query(self.game_state_manager.camera.view_rect());
            let mut renderables_sorted: Vec<(u8, Entity)> = Vec::new();
            for entity in visible_entities {
                if let Some(renderable) = self.game_state_manager.world.renderables.get(&entity) 
                    && self.game_state_manager.world.positions.contains_key(&entity) { renderables_sorted.push((renderable.z_index, entity)); }
            }
            renderables_sorted.sort_by(|a, b| b.0.cmp(&a.0));

            for (_, entity) in renderables_sorted {
                let mut draw_pos = crate::math::Vector2D::default();
                if let Some(curr_pos) = self.game_state_manager.world.positions.get(&entity) {
                    draw_pos = curr_pos.0;
                    if let Some(prev_pos) = self.game_state_manager.world.previous_positions.get(&entity) {
                        draw_pos.x = prev_pos.0.x * (1.0 - alpha) + curr_pos.0.x * alpha;
                        draw_pos.y = prev_pos.0.y * (1.0 - alpha) + curr_pos.0.y * alpha;
                    }
                }
                if let (Some(renderable), Some(animation)) = (self.game_state_manager.world.renderables.get(&entity), self.game_state_manager.world.animations.get(&entity))
                    && let (Some(texture_name), Some(frame_rect)) = (animation.controller.current_texture_name(), animation.controller.current_frame_rect()) {
                        self.renderer.draw_sprite(crate::renderer::SpriteDrawParams { pos: draw_pos, size: (renderable.width, renderable.height), offsets: (renderable.horizontal_offset, renderable.vertical_offset), texture_name, frame_rect, color_mod: None, rotation: renderable.rotation, flip_horizontal: renderable.flip_horizontal, flip_vertical: renderable.flip_vertical }, &mut self.texture_manager, &self.game_state_manager.camera).unwrap_or_else(|e| eprintln!("Failed to draw sprite: {}", e));
                    }
            }
            self.game_state_manager.benchmarker.pop();

            self.game_state_manager.benchmarker.push("Render.GUI");
            self.game_state_manager.draw(&mut self.renderer, &self.texture_manager, &self.config, &self._game_config, &self.input_state, self.frame_count, self.fps, self.show_debug_info, alpha)?;
            self.game_state_manager.benchmarker.pop();

            // 6. Physical presentation to the GPU and finalize frame telemetry.
            self.game_state_manager.benchmarker.push("Wait.VSync");
            self.renderer.present();
            self.game_state_manager.benchmarker.pop(); 
            self.game_state_manager.benchmarker.end_frame(); 
            self.frame_count += 1;
        }
        
        // 7. Session cleanup and write performance logs to disk.
        Ok(())
    }
}
