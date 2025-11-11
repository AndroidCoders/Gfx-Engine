// src/texture_manager.rs

//! Manages loading, storing, and retrieving textures.
//!
//! This module provides a centralized `TextureManager` to handle all texture
//! assets, preventing redundant loads and providing a simple interface for
//! accessing textures by a unique identifier.

use sdl3::render::{Texture, TextureCreator, BlendMode};
use sdl3::video::WindowContext;
use sdl3::surface::Surface;
use image::ImageReader;
use std::collections::HashMap;
use std::path::Path;

/// A manager for loading and storing all game textures.
///
/// It holds a hash map of `Texture` objects, indexed by a unique string name.
/// This allows other parts of the engine to access textures without having to
/// manage loading or lifetimes.
pub struct TextureManager {
    textures: HashMap<String, Texture>,
}

impl TextureManager {
    /// Creates a new, empty `TextureManager`.
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    /// Loads a texture from a file and stores it in the manager.
    ///
    /// This method uses the `image` crate to load the file from disk, converts
    /// it to an SDL `Surface`, and then creates an SDL `Texture` from it.
    ///
    /// # Arguments
    ///
    /// * `path` - The file path of the image to load.
    /// * `name` - The unique string identifier to associate with the loaded texture.
    /// * `texture_creator` - A reference to the SDL `TextureCreator`.
    ///
    /// # Errors
    ///
    /// Returns an error string if the image fails to load, decode, or be
    /// converted into an SDL texture.
    pub fn load(&mut self, path: &str, name: &str, texture_creator: &TextureCreator<WindowContext>) -> Result<(), String> {
        let path = Path::new(path);
        let image = ImageReader::open(path).map_err(|e| e.to_string())?.decode().map_err(|e| e.to_string())?.to_rgba8();
        let (width, height) = image.dimensions();
        let mut surface = Surface::new(width, height, sdl3::pixels::PixelFormatEnum::ABGR8888.into()).map_err(|e| e.to_string())?;
        surface.with_lock_mut(|pixels| {
            pixels.copy_from_slice(&image);
        });
        let mut texture = texture_creator.create_texture_from_surface(&surface).map_err(|e| e.to_string())?;
        texture.set_blend_mode(BlendMode::Blend);
        self.textures.insert(name.to_string(), texture);
        Ok(())
    }

    /// Retrieves a reference to a loaded texture by its identifier.
    ///
    /// # Arguments
    ///
    /// * `name` - The string identifier of the texture to retrieve.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the `Texture` if it exists, otherwise `None`.
    pub fn get(&self, name: &str) -> Option<&Texture> {
        self.textures.get(name)
    }
}