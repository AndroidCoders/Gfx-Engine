//! # Synchronization: Menu Logic
//! 
//! This module acts as the "Controller" for the user interface. It translates 
//! user input facts into navigation and state changes within the menu 
//! system, and provides the rendering routine for the visual layout.

use crate::input::{InputState, InputAction};
use crate::menu::{MenuState, RuntimeMenuItemType, MenuAction};
use crate::renderer::{Renderer, TextRenderParams};
use crate::font_manager::FontManager;
use sdl3::pixels::Color;

/// A system responsible for updating menu state and drawing the interface.
pub struct SystemMenu;

impl SystemMenu {
    /// Interprets input state to navigate menu indices and modify selector values.
    ///
    /// ⚠️ **Hotpath**: Called every frame when in Menu state.
    pub fn update(
        &mut self,
        menu_state: &mut MenuState,
        input_state: &InputState,
    ) -> Option<MenuAction> {
        // 1. Process vertical navigation facts (Up/Down).
        if input_state.is_action_just_pressed(InputAction::Up) {
            if menu_state.selected_index > 0 { menu_state.selected_index -= 1; }
            else { menu_state.selected_index = menu_state.items.len() - 1; }
        } else if input_state.is_action_just_pressed(InputAction::Down) {
            if menu_state.selected_index < menu_state.items.len() - 1 { menu_state.selected_index += 1; }
            else { menu_state.selected_index = 0; }
        }

        // 2. Process interaction with the currently selected item.
        if let Some(selected_item) = menu_state.items.get_mut(menu_state.selected_index) {
            match &mut selected_item.item_type {
                RuntimeMenuItemType::Selector { options, current_index, .. } => {
                    // Handle horizontal modifications for list selectors.
                    if input_state.is_action_just_pressed(InputAction::MoveLeft) {
                        if *current_index > 0 { *current_index -= 1; }
                        else { *current_index = options.len() - 1; }
                    } else if input_state.is_action_just_pressed(InputAction::MoveRight) {
                        if *current_index < options.len() - 1 { *current_index += 1; }
                        else { *current_index = 0; }
                    }
                }
                RuntimeMenuItemType::Action { action } => {
                    // Handle execution of semantic actions (Jump/Accept button).
                    if input_state.is_action_just_pressed(InputAction::Jump) { 
                        return Some(action.clone());
                    }
                }
            }
        }

        None
    }

    /// Renders the full-screen menu overlay based on the current active screen configuration.
    pub fn draw(
        &mut self,
        renderer: &mut Renderer,
        menu_state: &MenuState,
        font_manager: &FontManager,
    ) -> Result<(), String> {
        let config = &menu_state.config;
        let screen_config = config.screens.get(&menu_state.current_screen).ok_or("Current screen not found in config")?;

        // 1. Render a semi-transparent black scrim to dim the background.
        let (width, height) = renderer.output_size();
        let screen_rect = sdl3::rect::Rect::new(0, 0, width, height);
        renderer.fill_rect(&screen_rect, Color::RGBA(0, 0, 0, 150))?;
        
        // 2. Render the screen title with a stylistic drop shadow.
        let title_color = Color::RGB(255, 255, 255); 
        let shadow_color = Color::RGB(0, 0, 0);
        let (title_w, _) = font_manager.measure_text("debug", &screen_config.title, config.font_size as f32).unwrap_or((0, 0));
        let title_x = (width as i32 - title_w as i32) / 2;

        renderer.render_text(font_manager, TextRenderParams {
            text: &screen_config.title, x: title_x + 4, y: screen_config.title_y + 4,
            font_size: config.font_size as f32, scale: 1.0, color: shadow_color,
        })?;
        renderer.render_text(font_manager, TextRenderParams {
            text: &screen_config.title, x: title_x, y: screen_config.title_y,
            font_size: config.font_size as f32, scale: 1.0, color: title_color,
        })?;

        // 3. Iterate over items and render their labels and current values.
        let mut y = screen_config.start_y;
        for (index, item) in menu_state.items.iter().enumerate() {
            let is_selected = index == menu_state.selected_index;
            let color = if is_selected {
                Color::RGB(config.selected_color[0], config.selected_color[1], config.selected_color[2])
            } else {
                Color::RGB(config.unselected_color[0], config.unselected_color[1], config.unselected_color[2])
            };

            let label_text = if is_selected { format!("> {}", item.label) } else { item.label.clone() };
            let (lw, _) = font_manager.measure_text("debug", &label_text, config.font_size as f32).unwrap_or((0, 0));
            let lx = (width as i32 - lw as i32) / 2;

            renderer.render_text(font_manager, TextRenderParams {
                text: &label_text, x: lx, y, font_size: config.font_size as f32, scale: 1.0, color,
            })?;

            if let RuntimeMenuItemType::Selector { options, current_index, .. } = &item.item_type {
                if let Some(option_text) = options.get(*current_index) {
                    let opt_text = format!("< {} >", option_text);
                    let (ow, _) = font_manager.measure_text("debug", &opt_text, config.font_size as f32).unwrap_or((0, 0));
                    let ox = (width as i32 - ow as i32) / 2;
                    renderer.render_text(font_manager, TextRenderParams {
                        text: &opt_text, x: ox, y: y + 70, font_size: config.font_size as f32, scale: 1.0, color,
                    })?;
                }
                y += 70 + 50;
            }
            y += screen_config.spacing;
        }

        Ok(())
    }
}
