//! # Concept: Engine Configuration
//! 
//! This module defines the authoritative schema for core engine settings.
//! It is responsible for decoding 'config.toml' and applying global 
//! resolution-independent scaling to all physical and spatial constants.

use serde::Deserialize;
use std::fs;

/// The root structure for the global application configuration.
#[derive(Deserialize, Clone)]
pub struct Config {
    pub window: WindowConfig,
    pub input: InputConfig,
    pub physics: PhysicsConfig,
    pub debug: DebugConfig,
    pub game: GameSettings,
}

#[derive(Deserialize, Clone)]
pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub virtual_width: u32,
    pub virtual_height: u32,
    pub fullscreen: bool,
    pub vsync: bool,
    pub scaling_quality: String,
    pub camera_tightness: f32,
    pub camera_slow_zone: f32,
    pub camera_fast_zone: f32,
    pub camera_vertical_snap_threshold: f32,
    pub camera_vertical_tightness: f32,
    pub camera_falling_tightness: f32,
    pub camera_falling_velocity_threshold: f32,
    pub camera_lookahead_distance: f32,
    pub camera_smoothing_speed: f32,
}

#[derive(Deserialize, Clone)]
pub struct DebugConfig {
    pub show_debug_info: bool,
    pub debug_draw_collision_boxes: bool,
    pub text_start_x: i32,
    #[allow(dead_code)]
    pub text_start_y: i32,
    pub text_line_spacing: i32,
}

#[derive(Deserialize, Clone)]
pub struct GameSettings {
    pub start_level: String,
}

#[derive(Deserialize, Clone)]
pub struct PhysicsConfig {
    pub gravity: f32,
    pub max_speed: f32,
    pub entity_max_fall_speed: f32,
    pub acceleration: f32,
    pub deceleration: f32,
    pub jump_strength: f32,
    pub jump_hold_force: f32,
    pub air_control_factor: f32,
    pub velocity_threshold: f32,
}

#[derive(Deserialize, Clone)]
pub struct InputConfig {
    pub left: String,
    pub right: String,
    pub jump: String,
    #[serde(default = "default_key_up")] pub up: String,
    #[serde(default = "default_key_down")] pub down: String,
    pub quit: String,
    pub debug_toggle: String,
    #[serde(default = "default_key_f5")] pub record_toggle: String,
    #[serde(default = "default_key_f6")] pub save_replay: String,
}

fn default_key_up() -> String { "Up".to_string() }
fn default_key_down() -> String { "Down".to_string() }
fn default_key_f5() -> String { "F5".to_string() }
fn default_key_f6() -> String { "F6".to_string() }

/// Decodes the engine configuration from disk.
pub fn load_config() -> Result<Config, String> {
    let config_str = fs::read_to_string("config.toml").map_err(|e| e.to_string())?;
    let config: Config = toml::from_str(&config_str).map_err(|e| e.to_string())?;
    Ok(config)
}
