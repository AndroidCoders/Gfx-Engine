// src/input.rs

////! Handles user input and translates it into abstract game actions.

use sdl3::EventPump;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use std::collections::HashSet;
use crate::config::InputConfig;

/// Represents the abstract actions a player can take.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerAction {
    /// Move the player to the left.
    MoveLeft,
    /// Move the player to the right.
    MoveRight,
    /// Make the player jump.
    Jump,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DebugAction {
    ToggleDebugInfo,
}

/// Holds the current state of all player actions.
#[derive(Debug, Default)]
pub struct InputState {
    /// A set of all actions that are currently active (held down).
    active_actions: HashSet<PlayerAction>,
    /// A set of all actions that were just pressed in the current frame.
    just_pressed_actions: HashSet<PlayerAction>,
    /// A set of all debug actions that were just pressed in the current frame.
    just_pressed_debug_actions: HashSet<DebugAction>,
}

impl InputState {
    /// Checks if a specific action is currently active (held down).
    pub fn is_action_active(&self, action: PlayerAction) -> bool {
        self.active_actions.contains(&action)
    }

    /// Checks if a specific action was just pressed in the current frame.
    pub fn is_action_just_pressed(&self, action: PlayerAction) -> bool {
        self.just_pressed_actions.contains(&action)
    }

    /// Checks if a specific debug action was just pressed in the current frame.
    pub fn is_debug_action_just_pressed(&self, action: DebugAction) -> bool {
        self.just_pressed_debug_actions.contains(&action)
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
        // Clear the just-pressed actions at the beginning of the frame.
        input_state.just_pressed_actions.clear();
        input_state.just_pressed_debug_actions.clear();

        for event in event_pump.poll_iter() {
            println!("Event: {:?}", event);
            match event {
                Event::Quit { .. } => return false,
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    if keycode.name() == self.input_config.quit {
                        return false;
                    }
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
        let player_action = if keycode.name() == self.input_config.left {
            Some(PlayerAction::MoveLeft)
        } else if keycode.name() == self.input_config.right {
            Some(PlayerAction::MoveRight)
        } else if keycode.name() == self.input_config.jump {
            Some(PlayerAction::Jump)
        } else {
            None
        };

        if let Some(action) = player_action {
            if is_down {
                // If the action is not already active, it's a "just pressed" event.
                if !input_state.active_actions.contains(&action) {
                    input_state.just_pressed_actions.insert(action);
                }
                input_state.active_actions.insert(action);
            } else {
                input_state.active_actions.remove(&action);
            }
        }

        let debug_action = if keycode.name() == self.input_config.debug_toggle {
            Some(DebugAction::ToggleDebugInfo)
        } else {
            None
        };

        if let Some(action) = debug_action {
            if is_down {
                input_state.just_pressed_debug_actions.insert(action);
            }
        }
    }
}