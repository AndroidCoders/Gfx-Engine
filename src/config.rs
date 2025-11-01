// src/config.rs

//! Loads and manages the engine's configuration from external files.

pub const PIXEL_SCALE: f32 = 4.0;

use crate::math::Vector2D;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Represents the top-level configuration for the entire application.
#[derive(Deserialize, Clone)]
pub struct Config {
    /// Window-related configuration.
    pub window: WindowConfig,
    /// Input configuration.
    pub input: InputConfig,
    /// Physics configuration.
    pub physics: PhysicsConfig,
    /// Debug configuration.
    pub debug: DebugConfig,
}

/// Holds all window-related configuration.
#[derive(Deserialize, Clone)]
pub struct WindowConfig {
    /// The title of the window.
    pub title: String,
    /// The width of the window in pixels.
    pub width: u32,
    /// The height of the window in pixels.
    pub height: u32,
    /// The virtual width of the game canvas.
    pub virtual_width: u32,
    /// The virtual height of the game canvas.
    pub virtual_height: u32,

    /// Whether the window should be fullscreen.
    #[allow(dead_code)]
    pub fullscreen: bool,
    /// Whether VSync is enabled.
    pub vsync: bool,
    /// The scaling quality for the renderer.
    pub scaling_quality: String,
    pub camera_tightness: f32,
    pub camera_slow_zone: f32,
    pub camera_fast_zone: f32,
    pub camera_vertical_snap_threshold: f32,
    pub camera_vertical_tightness: f32,
    pub camera_falling_tightness: f32,
    pub camera_falling_velocity_threshold: f32,
}

/// Configuration for debug settings.
#[derive(Deserialize, Clone)]
pub struct DebugConfig {
    pub show_debug_info: bool,
    pub debug_draw_collision_boxes: bool,
}


/// Represents the game-specific configuration.
#[derive(Deserialize, Clone)]
pub struct CollectibleConfig {
    pub width: u32,
    pub height: u32,
    pub draw_width: u32,
    pub draw_height: u32,
}

#[derive(Deserialize, Clone)]
pub struct GameConfig {
    pub player: PlayerConfig,
    pub world: WorldConfig,

    #[serde(default)]
    pub enemy: HashMap<String, EnemyConfig>,

    #[serde(default)]
    pub collectible: HashMap<String, CollectibleConfig>,

    #[serde(default)]
    pub animation: HashMap<String, AnimationConfig>,

    #[serde(default)]
    pub audio: HashMap<String, String>,

}

#[derive(Deserialize, Clone)]
pub struct EnemyConfig {
    pub width: u32,
    pub height: u32,
    pub speed: f32,
    pub draw_width: u32,
    pub draw_height: u32,
}

#[derive(Deserialize, Clone)]
pub struct PlayerConfig {
    pub start_pos: Vector2D,
    pub width: u32,
    pub height: u32,
    pub draw_width: u32,
    pub draw_height: u32,
    pub horizontal_draw_offset: i32,
    pub vertical_draw_offset: i32,
    pub respawn_pos: Vector2D,
}

#[derive(Deserialize, Clone)]
pub struct WorldConfig {
    #[allow(dead_code)]
    pub width: f32,
    pub death_plane_y: f32,
}




#[derive(Deserialize, Clone)]
pub struct AnimationConfig {
    pub texture: String,
    pub start_x: i32,
    pub start_y: i32,
    pub frame_width: u32,
    pub frame_height: u32,
    pub frame_count: u32,
    pub frame_duration: u32,
    pub loops: bool,
    pub frame_padding: Option<u32>,
}

/// Configuration for physics parameters.
#[derive(Deserialize, Clone)]
pub struct PhysicsConfig {
    pub gravity: f32,
    pub max_speed: f32,
    pub entity_max_fall_speed: f32,
    pub acceleration: f32,
    pub friction: f32,
    pub jump_strength: f32,
    pub jump_hold_force: f32,

}

/// Configuration for input key bindings.
#[derive(Deserialize, Clone)]
pub struct InputConfig {
    pub left: String,
    pub right: String,
    pub jump: String,
    pub quit: String,
    pub debug_toggle: String,
}

/// Loads the main application configuration from the "config.toml" file.
pub fn load_config() -> Result<Config, String> {
    let config_str = fs::read_to_string("config.toml").map_err(|e| e.to_string())?;
    let config: Config = toml::from_str(&config_str).map_err(|e| e.to_string())?;
    Ok(config)
}

/// Loads the game-specific configuration from the specified path.
#[allow(dead_code)]
pub fn load_game_config(path: &str) -> Result<GameConfig, String> {
    // TODO: Move "assets" to a config file (e.g., assets.base_path)
    let full_path = PathBuf::from("assets").join(path);
    let config_str = fs::read_to_string(&full_path).map_err(|e| e.to_string())?;
    let game_config: GameConfig = toml::from_str(&config_str).map_err(|e| e.to_string())?;
    Ok(game_config)
}