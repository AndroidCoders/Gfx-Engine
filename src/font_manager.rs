//! # Manager: Font Rendering (TTF)
//!
//! This module is the authority for typography. It provides an abstraction 
//! over the 'rusttype' crate to load TrueType fonts and render them into 
//! SDL surfaces, enabling crisp pixel-art text for UI and debugging.

use rusttype::{Font, Scale, point};
use sdl3::surface::Surface;
use sdl3::pixels::{Color, PixelFormatEnum};
use std::collections::HashMap;
use std::fs;

/// A manager for loading and rendering TrueType fonts.
pub struct FontManager {
    fonts: HashMap<String, Font<'static>>,
}

impl FontManager {
    /// Initializes an empty font cache.
    pub fn new() -> Self {
        Self { fonts: HashMap::new() }
    }

    /// Loads a .ttf file from disk into the cache.
    pub fn load(&mut self, name: &str, path: &str) -> Result<(), String> {
        // 1. Read the font file bytes from disk.
        let data = fs::read(path).map_err(|e| e.to_string())?;
        
        // 2. Leak the memory to create a 'static buffer required by rusttype for performance.
        let data: &'static [u8] = Box::leak(data.into_boxed_slice());
        
        // 3. Parse the bytes into a usable Font structure.
        let font = Font::try_from_bytes(data).ok_or("Failed to parse font data")?;
        self.fonts.insert(name.to_string(), font);
        Ok(())
    }

    /// Calculates the dimensions required to render a specific string.
    pub fn measure_text(&self, name: &str, text: &str, size: f32) -> Result<(u32, u32), String> {
        let font = self.fonts.get(name).ok_or(format!("Font '{}' not found", name))?;
        let scale = Scale::uniform(size);
        let v_metrics = font.v_metrics(scale);
        let offset = point(0.0, v_metrics.ascent);

        // 1. Layout the text to determine the relative positions of all glyphs.
        let glyphs: Vec<_> = font.layout(text, scale, offset).collect();

        // 2. Compute the cumulative width based on individual glyph bounding boxes.
        let width = glyphs.iter().fold(0.0, |acc, g| {
            if let Some(bb) = g.pixel_bounding_box() {
                f32::max(acc, bb.max.x as f32)
            } else {
                acc
            }
        }).ceil() as u32;
        
        // 3. Calculate fixed line height based on font metrics.
        let height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
        Ok((width, height))
    }

    /// Renders a string into a stand-alone pixel buffer (SDL Surface).
    pub fn render_surface(&self, name: &str, text: &str, size: f32, color: Color) -> Result<Surface<'static>, String> {
        // 1. Determine the necessary canvas size for the text.
        let (width, height) = self.measure_text(name, text, size)?;
        
        if width == 0 || height == 0 {
             return Surface::new(1, 1, PixelFormatEnum::ABGR8888.into()).map_err(|e| e.to_string());
        }

        // 2. Create a new software surface with an RGBA channel.
        let mut surface = Surface::new(width, height, PixelFormatEnum::ABGR8888.into())
            .map_err(|e| e.to_string())?;

        let font = self.fonts.get(name).ok_or(format!("Font '{}' not found", name))?;
        let scale = Scale::uniform(size);
        let v_metrics = font.v_metrics(scale);
        let offset = point(0.0, v_metrics.ascent);
        let glyphs: Vec<_> = font.layout(text, scale, offset).collect();

        // 3. Manually rasterize each glyph into the target pixel buffer.
        surface.with_lock_mut(|buffer: &mut [u8]| {
            for g in glyphs {
                if let Some(bb) = g.pixel_bounding_box() {
                    // Iterate over every pixel affected by the glyph's coverage.
                    g.draw(|x, y, v| {
                        let x = x as i32 + bb.min.x;
                        let y = y as i32 + bb.min.y;
                        if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                            let alpha = (v * 255.0) as u8;
                            if alpha > 0 {
                                let index = (y as usize * width as usize + x as usize) * 4;
                                
                                // Map the font coverage to the surface's RGBA byte array.
                                buffer[index] = color.r;
                                buffer[index + 1] = color.g;
                                buffer[index + 2] = color.b;
                                buffer[index + 3] = alpha; 
                            }
                        }
                    });
                }
            }
        });

        Ok(surface)
    }
}

impl Default for FontManager {
    fn default() -> Self {
        Self::new()
    }
}
