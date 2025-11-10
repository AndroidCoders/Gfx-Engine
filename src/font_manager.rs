// src/font_manager.rs

//! Manages loading and storing fonts.

use sdl3_ttf::TtfContext;
use sdl3_ttf::Font;
use std::collections::HashMap;
use std::path::Path;

/// Manages all loaded fonts for the application.
///
/// This struct holds a hash map of fonts, where each font is associated with a
/// unique string identifier.
pub struct FontManager<'a> {
    _ttf_context: &'a TtfContext,
    fonts: HashMap<String, Font<'a, 'a>>,
}

impl<'a> FontManager<'a> {
    /// Creates a new, empty `FontManager`.
    ///
    /// # Arguments
    ///
    /// * `_ttf_context` - A reference to the SDL TTF context.
    pub fn new(_ttf_context: &'a TtfContext) -> Self {
        Self {
            _ttf_context,
            fonts: HashMap::new(),
        }
    }

    /// Loads a font from a file and adds it to the manager.
    ///
    /// # Arguments
    ///
    /// * `path` - The file path to the font.
    /// * `name` - A unique string identifier for the font.
    /// * `point_size` - The point size to load the font at.
    ///
    /// # Errors
    ///
    /// Returns an error string if the font fails to load.
    pub fn load(&mut self, path: &str, name: &str, point_size: u16) -> Result<(), String> {
        let path = Path::new(path);
        let font = self._ttf_context.load_font(path, point_size)?;
        self.fonts.insert(name.to_string(), font);
        Ok(())
    }

    /// Retrieves a reference to a loaded font by its identifier.
    ///
    /// # Arguments
    ///
    /// * `name` - The string identifier of the font to retrieve.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the `Font` if it exists, otherwise `None`.
    pub fn get(&self, name: &str) -> Option<&Font> {
        self.fonts.get(name)
    }
}
