//! # Concept: Menu Data Structures
//! 
//! This module defines the state and items for the application's user interface. 
//! It provides the runtime models for menu screens, actions, and interactive 
//! selectors, bridging the static configuration with the Menu System logic.

use crate::config::{MenuItemType, MenuConfig};

/// # Concept: Menu Action
/// Represents a semantic command triggered by a user's menu selection.
#[derive(Debug, Clone, PartialEq)]
pub enum MenuAction {
    StartGame,
    Quit,
    Navigate(String),
    SelectCharacter(usize),
    None,
}

impl From<&str> for MenuAction {
    /// Translates configuration strings into concrete runtime actions.
    fn from(s: &str) -> Self {
        // Handle parameterized actions like 'Goto(screen_name)'.
        if let Some(target) = s.strip_prefix("Goto(") {
             let target = target.trim_end_matches(')');
             return MenuAction::Navigate(target.to_string());
        }
        
        if let Some(val) = s.strip_prefix("SelectChar(") {
            let val = val.trim_end_matches(')');
            if let Ok(idx) = val.parse::<usize>() {
                return MenuAction::SelectCharacter(idx);
            }
        }

        // Handle simple atom actions.
        match s {
            "StartGame" => MenuAction::StartGame,
            "Quit" => MenuAction::Quit,
            _ => MenuAction::None,
        }
    }
}

/// A single interactive element within a menu screen.
#[derive(Debug, Clone)]
pub struct MenuItem {
    pub label: String,
    pub item_type: RuntimeMenuItemType,
}

/// Specialized state for different types of interactive items.
#[derive(Debug, Clone)]
pub enum RuntimeMenuItemType {
    Action { action: MenuAction },
    Selector { 
        options: Vec<String>, 
        current_index: usize, 
        #[allow(dead_code)]
        variable_name: String 
    },
}

/// The persistent state of the active menu interface.
#[derive(Debug, Clone)]
pub struct MenuState {
    pub items: Vec<MenuItem>,
    pub selected_index: usize,
    pub config: MenuConfig,
    pub current_screen: String,
}

impl MenuState {
    /// Initializes the menu state and loads the primary entry screen.
    pub fn new(config: &MenuConfig) -> Self {
        let mut state = Self {
            items: Vec::new(),
            selected_index: 0,
            config: config.clone(),
            current_screen: String::new(),
        };
        state.load_screen("main");
        state
    }

    /// Rebuilds the runtime item list based on a configuration screen key.
    pub fn load_screen(&mut self, screen_name: &str) {
        // 1. Locate the screen definition in the global menu configuration.
        if let Some(screen_config) = self.config.screens.get(screen_name) {
            self.current_screen = screen_name.to_string();
            self.selected_index = 0;
            
            // 2. Transform static item configs into live runtime models.
            self.items = screen_config.items.iter().map(|item_config| {
                let item_type = match &item_config.item_type {
                    MenuItemType::Action { action } => RuntimeMenuItemType::Action { 
                        action: MenuAction::from(action.as_str()) 
                    },
                    MenuItemType::Selector { options, variable } => RuntimeMenuItemType::Selector {
                        options: options.clone(),
                        current_index: 0,
                        variable_name: variable.clone(),
                    },
                };
                MenuItem {
                    label: item_config.label.clone(),
                    item_type,
                }
            }).collect();
        } else {
            eprintln!("Error: Menu screen '{}' not found in config.", screen_name);
        }
    }
}