// src/font_manager.rs

//! Manages loading and storing fonts.

use sdl3_ttf::TtfContext;
use sdl3_ttf::Font;
use std::collections::HashMap;
use std::path::Path;

pub struct FontManager<'a> {
    _ttf_context: &'a TtfContext,
    fonts: HashMap<String, Font<'a, 'a>>,
}

impl<'a> FontManager<'a> {
    pub fn new(_ttf_context: &'a TtfContext) -> Self {
        Self {
            _ttf_context,
            fonts: HashMap::new(),
        }
    }

    pub fn load(&mut self, path: &str, name: &str, point_size: u16) -> Result<(), String> {
        let path = Path::new(path);
        let font = self._ttf_context.load_font(path, point_size)?;
        self.fonts.insert(name.to_string(), font);
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&Font> {
        self.fonts.get(name)
    }
}
