//! # Concept: Game-Specific Configuration
//! 
//! This module defines the authoritative schema for gameplay-level data.
//! It is responsible for decoding 'game_config.toml' and managing 
//! high-level metadata for players, enemies, animations, and the UI.

use crate::math::Vector2D;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

/// The root structure for all game-specific data and asset registries.
#[derive(Deserialize, Clone)]
pub struct GameConfig {
    pub player: PlayerConfig,
    pub world: WorldConfig,
    #[serde(default)] pub gameplay: GameplayConfig,
    #[serde(default)] pub animation: HashMap<String, AnimationConfig>,
    #[serde(default)] pub audio: HashMap<String, String>,
    #[serde(default)] pub sound_events: HashMap<String, String>,
    #[serde(default)] pub soundtrack_properties: HashMap<String, SoundtrackConfig>,
    #[serde(default)] pub textures: HashMap<String, String>,
    #[serde(default)] pub prefabs: HashMap<String, PrefabConfig>,
    #[serde(default)] pub enemy_behavior: HashMap<String, EnemyBehaviorConfig>,
    #[serde(default)] pub menu: MenuConfig,
    #[serde(default)] pub parallax: ParallaxConfig,
    #[serde(default)] pub ui: UIConfig,
}

/// # Concept: HUD Configuration
#[derive(Deserialize, Clone)]
pub struct UIConfig {
    #[serde(default = "default_hearts_pos")] pub hearts_pos: UIPosition,
    #[serde(default = "default_coins_pos")] pub coins_pos: UIPosition,
}

impl Default for UIConfig {
    fn default() -> Self {
        Self {
            hearts_pos: default_hearts_pos(),
            coins_pos: default_coins_pos(),
        }
    }
}

#[derive(Deserialize, Clone, Copy)] pub struct UIPosition { pub x: i32, pub y: i32 }

fn default_hearts_pos() -> UIPosition { UIPosition { x: 20, y: 20 } }
fn default_coins_pos() -> UIPosition { UIPosition { x: 20, y: 100 } }

/// # Concept: Parallax Configuration
#[derive(Deserialize, Clone, Default)] pub struct ParallaxConfig { pub layers: Vec<ParallaxLayerConfig> }
#[derive(Deserialize, Clone)] pub struct ParallaxLayerConfig { pub texture: String, pub z_index: u8, pub scroll_speed_x: f32, pub scroll_speed_y: f32 }

/// # Concept: Soundtrack Metadata
#[derive(Deserialize, Clone)] pub struct SoundtrackConfig { pub bpm: Option<f32> }

/// # Concept: Menu Configuration
#[derive(Deserialize, Clone, Default, Debug)]
pub struct MenuConfig {
    pub font_size: u32,
    #[allow(dead_code)]
    pub item_spacing: i32,
    pub selected_color: [u8; 3],
    pub unselected_color: [u8; 3],
    #[serde(default)] pub screens: HashMap<String, MenuScreenConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MenuScreenConfig {
    pub title: String, #[allow(dead_code)] pub title_x: i32, pub title_y: i32,
    #[allow(dead_code)] pub start_x: i32, pub start_y: i32, pub spacing: i32,
    pub items: Vec<MenuItemConfig>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct MenuItemConfig { pub label: String, #[serde(flatten)] pub item_type: MenuItemType }

#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum MenuItemType { Action { action: String }, Selector { options: Vec<String>, variable: String } }

/// # Concept: Enemy Behavior Logic
#[derive(Deserialize, Clone)]
pub struct EnemyBehaviorConfig {
    #[allow(dead_code)] pub jump_interval: f32, pub jump_strength: f32,
    pub active_beats: u32, pub rest_beats: u32,
    #[allow(dead_code)] pub jump_sound: String, #[allow(dead_code)] pub rhythm_offset: f32,
    #[serde(default = "default_beats_per_jump")] pub beats_per_jump: u32,
}

fn default_beats_per_jump() -> u32 { 1 }

/// # Concept: Entity Prefabs
#[derive(Deserialize, Clone)] pub struct PrefabConfig { pub components: Vec<ComponentConfig> }

#[derive(Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ComponentConfig {
    Position, Velocity { x: f32, y: f32 }, Acceleration { x: f32, y: f32 },
    Renderable { draw_width: u32, draw_height: u32, z_index: u8, #[serde(default)] horizontal_offset: i32, #[serde(default)] vertical_offset: i32 },
    Animation { animations: Vec<String>, initial_animation: String },
    Collision { width: u32, height: u32 }, Gravity,
    Patrol { speed: f32, #[serde(default = "default_anim_prefix")] anim_prefix: String },
    EnemyTag, GoldCoin, Goal, StateComponent { initial_state: String },
}

fn default_anim_prefix() -> String { "enemy".to_string() }

/// # Concept: Player Parameters
#[derive(Deserialize, Clone)]
pub struct PlayerConfig {
    pub start_pos: Vector2D, pub width: u32, pub height: u32,
    pub draw_width: u32, pub draw_height: u32,
    pub horizontal_draw_offset: i32, pub vertical_draw_offset: i32,
    pub respawn_pos: Vector2D, pub lives: u32, pub max_health: u32,
}

/// # Concept: World Parameters
#[derive(Deserialize, Clone)] pub struct WorldConfig { pub width: f32, pub death_plane_y: f32 }

/// # Concept: Global Gameplay Settings
#[derive(Deserialize, Clone)]
pub struct GameplayConfig {
    pub stomp_bounce_velocity: f32, pub damage_knockback_force: f32,
    pub damage_invincibility_duration: f32, pub respawn_invincibility_duration: f32,
    pub game_over_duration: f32, pub game_over_texture: String,
    pub explosion: ExplosionConfig, pub audio: AudioSettingsConfig,
}

impl Default for GameplayConfig {
    fn default() -> Self {
        Self {
            stomp_bounce_velocity: -4.0,
            damage_knockback_force: 5.0,
            damage_invincibility_duration: 1.5,
            respawn_invincibility_duration: 2.0,
            game_over_duration: 3.0,
            game_over_texture: "game_over_3".to_string(),
            explosion: ExplosionConfig::default(),
            audio: AudioSettingsConfig::default(),
        }
    }
}

#[derive(Deserialize, Clone, Default)] pub struct AudioSettingsConfig { pub max_hearing_distance: f32, pub volume_falloff_power: f32 }
#[derive(Deserialize, Clone)] pub struct ExplosionConfig { pub width: u32, pub height: u32, pub horizontal_offset: i32, pub vertical_offset: i32, pub z_index: u8, pub animation_name: String }

impl Default for ExplosionConfig {
    fn default() -> Self {
        Self {
            width: 96,
            height: 96,
            horizontal_offset: -48,
            vertical_offset: -48,
            z_index: 101,
            animation_name: "explosion".to_string(),
        }
    }
}

/// # Concept: Animation Metadata
#[derive(Deserialize, Clone)]
pub struct AnimationConfig {
    pub texture: String, pub start_x: i32, pub start_y: i32,
    pub frame_width: u32, pub frame_height: u32, pub frame_count: u32,
    pub frame_duration: u32, pub loops: bool, pub frame_padding: Option<u32>,
    pub reverse: Option<bool>,
}

/// Decodes the game configuration from assets.
pub fn load_game_config(path: &str) -> Result<GameConfig, String> {
    let config_str = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let game_config: GameConfig = toml::from_str(&config_str).map_err(|e| e.to_string())?;
    Ok(game_config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_prefab_integrity() {
        // Load the actual game config
        let config = load_game_config("assets/game_config.toml").expect("Failed to load game_config.toml for validation");

        for (name, prefab) in config.prefabs {
            let mut has_patrol = false;
            let mut has_accel = false;
            let mut has_animation = false;
            let mut has_renderable = false;

            for component in prefab.components {
                match component {
                    ComponentConfig::Patrol { .. } => has_patrol = true,
                    ComponentConfig::Acceleration { .. } => has_accel = true,
                    ComponentConfig::Animation { .. } => has_animation = true,
                    ComponentConfig::Renderable { .. } => has_renderable = true,
                    _ => {}
                }
            }

            // Rule 1: Physics Dependency
            // If an entity is configured to Patrol, it MUST have Acceleration to physically move.
            if has_patrol && !has_accel {
                panic!("Prefab Integrity Error: '{}' has a [Patrol] component but is missing [Acceleration]. It will be stuck!", name);
            }

            // Rule 2: Visual Dependency
            // If an entity has Animations, it MUST have a Renderable component to display them.
            if has_animation && !has_renderable {
                 panic!("Prefab Integrity Error: '{}' has [Animation] but is missing [Renderable]. It will be invisible!", name);
            }
        }
    }
}