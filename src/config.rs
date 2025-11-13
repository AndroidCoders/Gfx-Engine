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

/// Represents the top-level configuration for the entire application, loaded from `config.toml`.
#[derive(Deserialize, Clone)]
pub struct Config {
    /// Window-related configuration.
    pub window: WindowConfig,
    /// Input bindings.
    pub input: InputConfig,
    /// Global physics constants.
    pub physics: PhysicsConfig,
    /// Debugging-related settings.
    pub debug: DebugConfig,
    /// Game-specific settings.
    pub game: GameSettings,
}

/// Holds all window-related configuration.
#[derive(Deserialize, Clone)]
pub struct WindowConfig {
    /// The title of the application window.
    pub title: String,
    /// The width of the window in pixels.
    pub width: u32,
    /// The height of the window in pixels.
    pub height: u32,
    /// The internal resolution width of the game canvas before scaling.
    pub virtual_width: u32,
    /// The internal resolution height of the game canvas before scaling.
    pub virtual_height: u32,
    /// Whether the window should open in fullscreen mode.
    #[allow(dead_code)]
    pub fullscreen: bool,
    /// Whether VSync is enabled to prevent screen tearing.
    pub vsync: bool,
    /// The scaling algorithm used by the renderer (e.g., "pixelart", "linear").
    pub scaling_quality: String,
    /// The tightness of the camera's smooth follow movement.
    pub camera_tightness: f32,
    /// The percentage of the screen where the player can move without the camera following.
    pub camera_slow_zone: f32,
    /// The percentage of the screen edge that triggers a faster camera follow.
    pub camera_fast_zone: f32,
    /// The velocity threshold for the camera to snap vertically.
    pub camera_vertical_snap_threshold: f32,
    /// The tightness of the camera's vertical movement.
    pub camera_vertical_tightness: f32,
    /// The tightness of the camera's movement when the player is falling.
    pub camera_falling_tightness: f32,
    /// The downward velocity at which the falling camera tightness is triggered.
    pub camera_falling_velocity_threshold: f32,
    /// The distance the camera looks ahead of the player in the direction of movement.
    pub camera_lookahead_distance: f32,
}

/// Holds all debugging-related configuration.
#[derive(Deserialize, Clone)]
pub struct DebugConfig {
    /// Whether to display on-screen debug information.
    #[allow(dead_code)]
    pub show_debug_info: bool,
    /// Whether to draw collision boxes for all entities with a `Collision` component.
    pub debug_draw_collision_boxes: bool,
}

/// Holds game-specific settings from `config.toml`.
#[derive(Deserialize, Clone)]
pub struct GameSettings {
    /// The path to the initial level to load.
    pub start_level: String,
}

/// Represents the configuration for a collectible item, loaded from `game_config.toml`.
#[derive(Deserialize, Clone)]
pub struct CollectibleConfig {
    /// The width of the collectible's collision box in world units.
    pub width: u32,
    /// The height of the collectible's collision box in world units.
    pub height: u32,
    /// The width of the collectible's sprite for rendering, in world units.
    pub draw_width: u32,
    /// The height of the collectible's sprite for rendering, in world units.
    pub draw_height: u32,
}

/// Represents the configuration for the level goal, loaded from `game_config.toml`.
#[derive(Deserialize, Clone, Default)]
pub struct GoalConfig {
    /// The width of the goal's sprite for rendering, in world units.
    pub draw_width: u32,
    /// The height of the goal's sprite for rendering, in world units.
    pub draw_height: u32,
    /// The vertical rendering offset of the goal's sprite.
    pub vertical_draw_offset: i32,
    /// The horizontal rendering offset of the goal's sprite.
    pub horizontal_draw_offset: i32,
}

/// Represents the game-specific configuration, loaded from `game_config.toml`.
#[derive(Deserialize, Clone)]
pub struct GameConfig {
    /// Player-specific configuration.
    pub player: PlayerConfig,
    /// World-specific configuration, like boundaries and death planes.
    pub world: WorldConfig,
    /// A map of configurations for different enemy types.
    #[serde(default)]
    pub enemy: HashMap<String, EnemyConfig>,
    /// A map of configurations for different collectible types.
    #[serde(default)]
    pub collectible: HashMap<String, CollectibleConfig>,
    /// A map of all sprite animations available in the game.
    #[serde(default)]
    pub animation: HashMap<String, AnimationConfig>,
    /// A map of all audio sound effects.
    #[serde(default)]
    pub audio: HashMap<String, String>,
    /// A map of sound events to sound names.
    #[serde(default)]
    pub sound_events: HashMap<String, String>,
    /// A map of texture names to their file paths.
    #[serde(default)]
    pub textures: HashMap<String, String>,
    /// Configuration for the level goal.
    #[serde(default)]
    pub goal: GoalConfig,
}

/// Represents the configuration for a specific type of enemy.
#[derive(Deserialize, Clone)]
pub struct EnemyConfig {
    /// The width of the enemy's collision box in world units.
    pub width: u32,
    /// The height of the enemy's collision box in world units.
    pub height: u32,
    /// The patrol speed of the enemy.
    pub speed: f32,
    /// The width of the enemy's sprite for rendering, in world units.
    pub draw_width: u32,
    /// The height of the enemy's sprite for rendering, in world units.
    pub draw_height: u32,
}

/// Represents the player's configuration.
#[derive(Deserialize, Clone)]
pub struct PlayerConfig {
    /// The player's starting position in the world.
    pub start_pos: Vector2D,
    /// The width of the player's collision box in world units.
    pub width: u32,
    /// The height of the player's collision box in world units.
    pub height: u32,
    /// The width of the player's sprite for rendering, in world units.
    pub draw_width: u32,
    /// The height of the player's sprite for rendering, in world units.
    pub draw_height: u32,
    /// The horizontal rendering offset of the player's sprite.
    pub horizontal_draw_offset: i32,
    /// The vertical rendering offset of the player's sprite.
    pub vertical_draw_offset: i32,
    /// The position where the player respawns after death.
    pub respawn_pos: Vector2D,
    /// The starting number of lives for the player.
    pub lives: u32,
}

/// Represents the world's configuration.
#[derive(Deserialize, Clone)]
pub struct WorldConfig {
    /// The total width of the world, used for camera boundaries.
    #[allow(dead_code)]
    pub width: f32,
    /// The y-coordinate below which an entity is considered to have fallen out of the world.
    pub death_plane_y: f32,
}




/// Represents the configuration for a single sprite animation.
#[derive(Deserialize, Clone)]
pub struct AnimationConfig {
    /// The path to the texture file containing the animation frames.
    pub texture: String,
    /// The x-coordinate of the top-left corner of the first frame on the sprite sheet.
    pub start_x: i32,
    /// The y-coordinate of the top-left corner of the first frame on the sprite sheet.
    pub start_y: i32,
    /// The width of a single frame in pixels.
    pub frame_width: u32,
    /// The height of a single frame in pixels.
    pub frame_height: u32,
    /// The total number of frames in the animation.
    pub frame_count: u32,
    /// The duration of each frame in game ticks (updates).
    pub frame_duration: u32,
    /// Whether the animation should loop back to the beginning when it ends.
    pub loops: bool,
    /// The spacing between frames on the sprite sheet, if any.
    pub frame_padding: Option<u32>,
}

/// Represents global physics constants.
#[derive(Deserialize, Clone)]
pub struct PhysicsConfig {
    /// The downward acceleration applied to entities with a `Gravity` component.
    pub gravity: f32,
    /// The maximum horizontal speed an entity can reach.
    pub max_speed: f32,
    /// The maximum downward speed an entity can reach.
    pub entity_max_fall_speed: f32,
    /// The rate at which an entity gains horizontal speed.
    pub acceleration: f32,
    /// The rate at which an entity loses horizontal speed when no input is given.
    pub deceleration: f32,
    /// The initial upward velocity applied for a jump.
    pub jump_strength: f32,
    /// The additional upward force applied when the jump button is held down.
    pub jump_hold_force: f32,

}

/// Represents input key bindings.
#[derive(Deserialize, Clone)]
pub struct InputConfig {
    /// The key for moving the player left.
    pub left: String,
    /// The key for moving the player right.
    pub right: String,
    /// The key for making the player jump.
    pub jump: String,
    /// The key for quitting the application.
    pub quit: String,
    /// The key for toggling the on-screen debug display.
    pub debug_toggle: String,
}

/// Loads the main application configuration from the "config.toml" file.
///
/// # Errors
///
/// This function will return an error if the file cannot be read or if the
/// TOML content cannot be parsed into the `Config` struct.
pub fn load_config() -> Result<Config, String> {
    let config_str = fs::read_to_string("config.toml").map_err(|e| e.to_string())?;
    let config: Config = toml::from_str(&config_str).map_err(|e| e.to_string())?;
    Ok(config)
}

/// Loads the game-specific configuration from the specified path.
///
/// # Errors
///
/// This function will return an error if the file cannot be read or if the
/// TOML content cannot be parsed into the `GameConfig` struct.
#[allow(dead_code)]
pub fn load_game_config(path: &str) -> Result<GameConfig, String> {
    // TODO: Move "assets" to a config file (e.g., assets.base_path)
    let full_path = PathBuf::from("assets").join(path);
    let config_str = fs::read_to_string(&full_path).map_err(|e| e.to_string())?;
    let game_config: GameConfig = toml::from_str(&config_str).map_err(|e| e.to_string())?;
    Ok(game_config)
}