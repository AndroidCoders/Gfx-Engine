// src/config.rs

//! This module defines the structures for loading and managing the engine's
//! configuration from external TOML files.

/// The scale factor for rendering pixels.
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
    /// The tightness of the camera's movement.
    pub camera_tightness: f32,
    /// The size of the camera's slow zone.
    pub camera_slow_zone: f32,
    /// The size of the camera's fast zone.
    pub camera_fast_zone: f32,
    /// The threshold for snapping the camera vertically.
    pub camera_vertical_snap_threshold: f32,
    /// The tightness of the camera's vertical movement.
    pub camera_vertical_tightness: f32,
    /// The tightness of the camera's movement when the player is falling.
    pub camera_falling_tightness: f32,
    /// The velocity threshold for triggering the falling camera tightness.
    pub camera_falling_velocity_threshold: f32,
}

/// Configuration for debug settings.
#[derive(Deserialize, Clone)]
pub struct DebugConfig {
    /// Whether to show debug information on the screen.
    #[allow(dead_code)]
    pub show_debug_info: bool,
    /// Whether to draw collision boxes for entities.
    pub debug_draw_collision_boxes: bool,
}


/// Represents the configuration for a collectible item.
#[derive(Deserialize, Clone)]
pub struct CollectibleConfig {
    /// The width of the collectible's collision box.
    pub width: u32,
    /// The height of the collectible's collision box.
    pub height: u32,
    /// The width of the collectible's sprite.
    pub draw_width: u32,
    /// The height of the collectible's sprite.
    pub draw_height: u32,
}

/// Represents the game-specific configuration.
#[derive(Deserialize, Clone)]
pub struct GameConfig {
    /// The player's configuration.
    pub player: PlayerConfig,
    /// The world's configuration.
    pub world: WorldConfig,
    /// A map of enemy configurations.
    #[serde(default)]
    pub enemy: HashMap<String, EnemyConfig>,
    /// A map of collectible configurations.
    #[serde(default)]
    pub collectible: HashMap<String, CollectibleConfig>,
    /// A map of animation configurations.
    #[serde(default)]
    pub animation: HashMap<String, AnimationConfig>,
    /// A map of audio configurations.
    #[serde(default)]
    pub audio: HashMap<String, String>,

}

/// Represents the configuration for an enemy.
#[derive(Deserialize, Clone)]
pub struct EnemyConfig {
    /// The width of the enemy's collision box.
    pub width: u32,
    /// The height of the enemy's collision box.
    pub height: u32,
    /// The speed of the enemy.
    pub speed: f32,
    /// The width of the enemy's sprite.
    pub draw_width: u32,
    /// The height of the enemy's sprite.
    pub draw_height: u32,
}

/// Represents the player's configuration.
#[derive(Deserialize, Clone)]
pub struct PlayerConfig {
    /// The player's starting position.
    pub start_pos: Vector2D,
    /// The width of the player's collision box.
    pub width: u32,
    /// The height of the player's collision box.
    pub height: u32,
    /// The width of the player's sprite.
    pub draw_width: u32,
    /// The height of the player's sprite.
    pub draw_height: u32,
    /// The horizontal offset of the player's sprite.
    pub horizontal_draw_offset: i32,
    /// The vertical offset of the player's sprite.
    pub vertical_draw_offset: i32,
    /// The player's respawn position.
    pub respawn_pos: Vector2D,
}

/// Represents the world's configuration.
#[derive(Deserialize, Clone)]
pub struct WorldConfig {
    /// The width of the world.
    #[allow(dead_code)]
    pub width: f32,
    /// The y-coordinate of the death plane.
    pub death_plane_y: f32,
}




/// Represents the configuration for an animation.
#[derive(Deserialize, Clone)]
pub struct AnimationConfig {
    /// The path to the animation's texture.
    pub texture: String,
    /// The x-coordinate of the first frame.
    pub start_x: i32,
    /// The y-coordinate of the first frame.
    pub start_y: i32,
    /// The width of each frame.
    pub frame_width: u32,
    /// The height of each frame.
    pub frame_height: u32,
    /// The number of frames in the animation.
    pub frame_count: u32,
    /// The duration of each frame in game ticks.
    pub frame_duration: u32,
    /// Whether the animation should loop.
    pub loops: bool,
    /// The padding between frames.
    pub frame_padding: Option<u32>,
}

/// Configuration for physics parameters.
#[derive(Deserialize, Clone)]
pub struct PhysicsConfig {
    /// The strength of gravity.
    pub gravity: f32,
    /// The maximum horizontal speed of an entity.
    pub max_speed: f32,
    /// The maximum vertical speed of an entity.
    pub entity_max_fall_speed: f32,
    /// The horizontal acceleration of an entity.
    pub acceleration: f32,
    /// The horizontal deceleration of an entity.
    pub deceleration: f32,
    /// The initial vertical velocity of a jump.
    pub jump_strength: f32,
    /// The force applied when holding the jump button.
    pub jump_hold_force: f32,

}

/// Configuration for input key bindings.
#[derive(Deserialize, Clone)]
pub struct InputConfig {
    /// The key for moving left.
    pub left: String,
    /// The key for moving right.
    pub right: String,
    /// The key for jumping.
    pub jump: String,
    /// The key for quitting the game.
    pub quit: String,
    /// The key for toggling debug information.
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