// src/input.rs

//! Handles user input and translates it into abstract game actions.

use sdl3::EventPump;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use std::collections::HashSet;
use crate::config::InputConfig;

/// Represents the abstract actions a player can take.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerAction {
    MoveLeft,
    MoveRight,
    Jump,
}

/// Holds the current state of all player actions.
#[derive(Debug, Default)]
pub struct InputState {
    active_actions: HashSet<PlayerAction>,
}

impl InputState {
    /// Checks if a specific action is currently active.
    pub fn is_action_active(&self, action: PlayerAction) -> bool {
        self.active_actions.contains(&action)
    }
}

/// The main input handler for the game.
pub struct InputHandler {
    input_config: InputConfig,
}

impl InputHandler {
    /// Creates a new `InputHandler` with the given configuration.
    pub fn new(input_config: InputConfig) -> Self {
        Self { input_config }
    }

    /// Processes SDL events and updates the `InputState`.
    pub fn process_events(&self, event_pump: &mut EventPump, input_state: &mut InputState) -> bool {
        input_state.active_actions.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return false,
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    self.handle_key_event(keycode, input_state, true);
                }
                Event::KeyUp { keycode: Some(keycode), .. } => {
                    self.handle_key_event(keycode, input_state, false);
                }
                _ => {},
            }
        }

        true
    }

    /// Handles a single key event and updates the input state.
    fn handle_key_event(&self, keycode: Keycode, input_state: &mut InputState, is_down: bool) {
        let action = if keycode.name() == self.input_config.left {
            Some(PlayerAction::MoveLeft)
        } else if keycode.name() == self.input_config.right {
            Some(PlayerAction::MoveRight)
        } else if keycode.name() == self.input_config.jump {
            Some(PlayerAction::Jump)
        } else {
            None
        };

        if let Some(action) = action {
            if is_down {
                input_state.active_actions.insert(action);
            } else {
                input_state.active_actions.remove(&action);
            }
        }
    }
}
