// src/level.rs

//! Manages loading and representing game levels.

use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Clone)]
pub struct LevelObject {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Deserialize, Clone)]
pub struct Level {
    pub objects: Vec<LevelObject>,
}

pub fn load_level(path: &str) -> Result<Level, String> {
    let level_str = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let level: Level = toml::from_str(&level_str).map_err(|e| e.to_string())?;
    Ok(level)
}