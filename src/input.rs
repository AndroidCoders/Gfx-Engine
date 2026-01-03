use crate::config::InputConfig;

use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::EventPump;
use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub enum InputAction {
    MoveLeft,
    MoveRight,
    Jump,
    Up,
    Down,
    Quit,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum DebugAction {
    ToggleDebugInfo,
    ToggleRecording,
    SaveReplay,
}

#[derive(Default, Clone)]
pub struct InputState {
    actions_pressed: HashSet<InputAction>,
    actions_just_pressed: HashSet<InputAction>,
    actions_just_released: HashSet<InputAction>,
    debug_actions_just_pressed: HashSet<DebugAction>,
}

impl InputState {
    pub fn is_action_pressed(&self, action: InputAction) -> bool {
        self.actions_pressed.contains(&action)
    }

    pub fn is_action_just_pressed(&self, action: InputAction) -> bool {
        self.actions_just_pressed.contains(&action)
    }



    pub fn is_debug_action_just_pressed(&self, action: DebugAction) -> bool {
        self.debug_actions_just_pressed.contains(&action)
    }

    pub fn get_pressed_actions(&self) -> Vec<InputAction> {
        self.actions_pressed.iter().copied().collect()
    }

    #[allow(dead_code)]
    pub fn get_pressed_actions_debug(&self) -> String {
        self.actions_pressed.iter().map(|a| format!("{:?}", a)).collect::<Vec<_>>().join(", ")
    }

    pub fn from_actions(actions: Vec<InputAction>) -> Self {
        let mut state = Self::default();
        for action in actions {
            state.actions_pressed.insert(action);
            // In playback, we might need to simulate "just pressed" if we want to be perfect.
            // For now, let's assume basic movement works with just "pressed".
            // If "just pressed" is needed (e.g. jumps), we need to compare with previous frame!
            // TODO: Handle just_pressed in Replay logic by tracking previous frame's simulated state.
        }
        state
    }

    pub fn calculate_deltas(&mut self, previous: &InputState) {
        for action in &self.actions_pressed {
            if !previous.actions_pressed.contains(action) {
                self.actions_just_pressed.insert(*action);
            }
        }
        for action in &previous.actions_pressed {
            if !self.actions_pressed.contains(action) {
                self.actions_just_released.insert(*action);
            }
        }
    }
}

pub struct InputHandler {
    key_bindings: HashMap<Keycode, InputAction>,
    debug_bindings: HashMap<Keycode, DebugAction>,
}

impl InputHandler {
    pub fn new(config: InputConfig) -> Self {
        let mut key_bindings = HashMap::new();
        
        // Helper to parse keycode string
        let parse_key = |s: &str| -> Option<Keycode> {
            Keycode::from_name(s)
        };

        if let Some(k) = parse_key(&config.left) { key_bindings.insert(k, InputAction::MoveLeft); }
        if let Some(k) = parse_key(&config.right) { key_bindings.insert(k, InputAction::MoveRight); }
        if let Some(k) = parse_key(&config.jump) { key_bindings.insert(k, InputAction::Jump); }
        
        // Menu Navigation Bindings
        // Note: Using Same keys for Game Move and Menu Move is common, 
        // or we can use Arrow Keys strictly for Menu.
        // For simplicity, let's map Arrows to UI actions explicitly if they aren't mapped to movement.
        // Actually, let's map config.up/down.
        if let Some(k) = parse_key(&config.up) { key_bindings.insert(k, InputAction::Up); }
        if let Some(k) = parse_key(&config.down) { key_bindings.insert(k, InputAction::Down); }
        if let Some(k) = parse_key(&config.quit) { key_bindings.insert(k, InputAction::Quit); }
        
        let mut debug_bindings = HashMap::new();
        if let Some(k) = parse_key(&config.debug_toggle) { debug_bindings.insert(k, DebugAction::ToggleDebugInfo); }
        if let Some(k) = parse_key(&config.record_toggle) { debug_bindings.insert(k, DebugAction::ToggleRecording); }
        if let Some(k) = parse_key(&config.save_replay) { debug_bindings.insert(k, DebugAction::SaveReplay); }

        Self {
            key_bindings,
            debug_bindings,
        }
    }

    pub fn process_events(&self, event_pump: &mut EventPump, input_state: &mut InputState) -> bool {
        // Clear "just" states
        input_state.actions_just_pressed.clear();
        input_state.actions_just_released.clear();
        input_state.debug_actions_just_pressed.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return false,
                Event::KeyDown { keycode: Some(keycode), repeat: false, .. } => {
                    // Check normal bindings
                    if let Some(&action) = self.key_bindings.get(&keycode) {
                        input_state.actions_pressed.insert(action);
                        input_state.actions_just_pressed.insert(action);
                    }
                    // Check debug bindings
                    if let Some(&action) = self.debug_bindings.get(&keycode) {
                        input_state.debug_actions_just_pressed.insert(action);
                    }
                }
                Event::KeyUp { keycode: Some(keycode), .. } => {
                    if let Some(&action) = self.key_bindings.get(&keycode) {
                        input_state.actions_pressed.remove(&action);
                        input_state.actions_just_released.insert(action);
                    }
                }
                _ => {}
            }
        }
        true
    }
}
