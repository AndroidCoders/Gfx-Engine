//! # Manager: Texture Assets
//!
//! This module provides the central authority for graphical assets. 
//! It handles the loading, decoding, and caching of image files, ensuring 
//! that textures are upscaled correctly for the engine's 1:1 pixel workspace.

use sdl3::render::{Texture, TextureCreator, BlendMode};
use sdl3::video::WindowContext;
use sdl3::surface::Surface;
use image::ImageReader;
use std::collections::HashMap;
use std::path::Path;

/// A repository for managing the lifecycle of SDL texture resources.
pub struct TextureManager {
    textures: HashMap<String, Texture>,
}

impl TextureManager {
    /// Initializes a new, empty asset manager.
    pub fn new() -> Self {
        Self { textures: HashMap::new() }
    }

    /// Decodes an image from disk and creates a GPU texture upscaled for high-res rendering.
    pub fn load(&mut self, path: &str, name: &str, texture_creator: &TextureCreator<WindowContext>) -> Result<(), String> {
        // 1. Open and decode the image file using the 'image' crate.
        let path_obj = Path::new(path);
        let image = ImageReader::open(path_obj).map_err(|e| e.to_string())?.decode().map_err(|e| e.to_string())?;
        
        let scaled_image = image.to_rgba8();

        // 3. Construct an SDL Surface from the raw pixel buffer.
        let (width, height) = scaled_image.dimensions();
        let mut surface = Surface::new(width, height, sdl3::pixels::PixelFormatEnum::ABGR8888.into()).map_err(|e| e.to_string())?;
        surface.with_lock_mut(|pixels| {
            pixels.copy_from_slice(&scaled_image);
        });

        // 4. Create the final GPU Texture and enforce Nearest-Neighbor scaling for the pixel-art aesthetic.
        let mut texture = texture_creator.create_texture_from_surface(&surface).map_err(|e| e.to_string())?;
        texture.set_blend_mode(BlendMode::Blend);
        
        unsafe {
            sdl3_sys::render::SDL_SetTextureScaleMode(texture.raw(), sdl3_sys::surface::SDL_SCALEMODE_NEAREST);
        }

        // 5. Store the texture in the cache, indexed by its unique identifier.
        self.textures.insert(name.to_string(), texture);
        Ok(())
    }

    /// Provides immutable access to a loaded texture.
    pub fn get(&self, name: &str) -> Option<&Texture> {
        self.textures.get(name)
    }

    /// Provides mutable access to a loaded texture (e.g., for color modulation).
    pub fn get_mut(&mut self, name: &str) -> Option<&mut Texture> {
        self.textures.get_mut(name)
    }
}

impl Default for TextureManager {
    fn default() -> Self {
        Self::new()
    }
}
