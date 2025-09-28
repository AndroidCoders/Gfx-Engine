// src/config.rs

//! Loads and manages the engine's configuration from external files.

use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

/// Represents the top-level configuration for the entire application.
#[derive(Deserialize, Clone)]
pub struct Config {
    /// Window-related configuration.
    pub window: WindowConfig,
    /// Input configuration.
    pub input: InputConfig,
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
    /// The background color of the window.
    pub background_color: [u8; 3],
    /// Whether the window should be fullscreen.
    #[allow(dead_code)]
    pub fullscreen: bool,
    /// Whether VSync is enabled.
    pub vsync: bool,
    /// The scaling quality for the renderer.
    pub scaling_quality: String,
}

#[derive(Deserialize, Clone)]
pub struct PlayerConfig {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

/// Represents the game-specific configuration (minimal for POC).
#[derive(Deserialize, Clone)]
pub struct GameConfig {
    pub player: PlayerConfig,
}

/// Configuration for input key bindings.
#[derive(Deserialize, Clone)]
pub struct InputConfig {
    pub left: String,
    pub right: String,
    pub jump: String,
    pub quit: String,
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
    let full_path = PathBuf::from("assets").join(path);
    let config_str = fs::read_to_string(&full_path).map_err(|e| e.to_string())?;
    let game_config: GameConfig = toml::from_str(&config_str).map_err(|e| e.to_string())?;
    Ok(game_config)
}