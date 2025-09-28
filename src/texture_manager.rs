// src/texture_manager.rs

//! Manages loading and storing textures.

use sdl3::render::{Texture, TextureCreator};
use sdl3::video::WindowContext;
use sdl3::surface::Surface;
use sdl3::pixels::PixelFormatEnum;
use image::ImageReader;
use std::collections::HashMap;
use std::path::Path;

pub struct TextureManager {
    textures: HashMap<String, Texture>,
}

impl TextureManager {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    pub fn load(&mut self, path: &str, name: &str, texture_creator: &TextureCreator<WindowContext>) -> Result<(), String> {
        let path = Path::new(path);
        let player_image = ImageReader::open(path).map_err(|e| e.to_string())?.decode().map_err(|e| e.to_string())?.to_rgba8();
        let (width, height) = player_image.dimensions();
        let mut player_surface = Surface::new(width, height, PixelFormatEnum::ABGR8888.into()).map_err(|e| e.to_string())?;
        player_surface.with_lock_mut(|pixels| {
            pixels.copy_from_slice(&player_image);
        });
        let player_texture = texture_creator.create_texture_from_surface(&player_surface).map_err(|e| e.to_string())?;
        self.textures.insert(name.to_string(), player_texture);
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&Texture> {
        self.textures.get(name)
    }
}