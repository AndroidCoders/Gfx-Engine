// src/config.rs

//! Loads and manages the engine's configuration from external files.

use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

/// Represents the top-level configuration for the entire application.
#[derive(Deserialize, Clone)]
pub struct Config {
    pub window: WindowConfig,
}

/// Holds all window-related configuration.
#[derive(Deserialize, Clone)]
pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub virtual_width: u32,
    pub virtual_height: u32,
    pub background_color: [u8; 3],
    pub fullscreen: bool,
    pub vsync: bool,
    pub scaling_quality: String,
}

/// Represents the game-specific configuration (minimal for POC).
#[derive(Deserialize, Clone)]
pub struct GameConfig {
    // No fields for minimal POC
}

/// Loads the main application configuration from the "config.toml" file.
pub fn load_config() -> Result<Config, String> {
    let config_str = fs::read_to_string("config.toml").map_err(|e| e.to_string())?;
    let config: Config = toml::from_str(&config_str).map_err(|e| e.to_string())?;
    Ok(config)
}

/// Loads the game-specific configuration from the specified path.
pub fn load_game_config(path: &str) -> Result<GameConfig, String> {
    let full_path = PathBuf::from("assets").join(path);
    let config_str = fs::read_to_string(&full_path).map_err(|e| e.to_string())?;
    let game_config: GameConfig = toml::from_str(&config_str).map_err(|e| e.to_string())?;
    Ok(game_config)
}