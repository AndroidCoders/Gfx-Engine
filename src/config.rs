use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub window: WindowConfig,
    pub renderer: RendererConfig,
        pub physics: PhysicsConfig,
        pub debug: DebugConfig,    pub objects: Vec<ObjectConfig>,
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
}

#[derive(Deserialize, Clone)]
pub struct RendererConfig {
    pub background_color: [u8; 3],
    pub object_color: [u8; 3],
}

#[derive(Deserialize, Clone)]
pub struct PhysicsConfig {
    pub updates_per_second: u32,
    pub damping_factor: f32,
    pub max_speed: f32,
    pub min_speed: f32,
}

#[derive(Deserialize, Clone)]
pub struct DebugConfig {
    pub enable_frame_capture: bool,
    pub output_directory: String,
    pub max_captured_frames: usize,
    pub frame_capture_interval: u32,
}

/// Configuration for an individual game object.
#[derive(Deserialize, Clone)]
pub struct ObjectConfig {
    pub x: f32,
    pub y: f32,
    pub width: u32,
    pub height: u32,
    pub vx: f32,
    pub vy: f32,
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&config_str)?;
    Ok(config)
}
