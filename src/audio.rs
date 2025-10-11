// src/audio.rs

//! Handles audio loading and playback using the Kira library.

use kira::{
    manager::{AudioManager, AudioManagerSettings, backend::DefaultBackend},
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
    tween::Tween,
    StartTime,
};
use std::collections::HashMap;
use std::path::Path;
use std::sync::mpsc;

/// Represents the types of audio events that can occur in the game.
pub enum AudioEvent {
    PlayerJumped,
    // Add other events as needed, e.g., EnemyHit, CollectCoin, BackgroundMusicStart
}

/// Manages loading and playing audio assets using Kira.
pub struct GameAudioManager {
    manager: AudioManager<DefaultBackend>,
    sounds: HashMap<String, StaticSoundData>,
    // We can add music handles, spatial emitters, etc., here later
    event_receiver: mpsc::Receiver<AudioEvent>,
    event_sender: mpsc::Sender<AudioEvent>,
}

impl GameAudioManager {
    /// Creates a new `GameAudioManager` and initializes Kira's audio manager.
    pub fn new() -> Result<Self, String> {
        let manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())
            .map_err(|e| format!("Failed to create Kira audio manager: {}", e))?;

        let (event_sender, event_receiver) = mpsc::channel();

        Ok(Self {
            manager,
            sounds: HashMap::new(),
            event_receiver,
            event_sender,
        })
    }

    /// Returns a sender for `AudioEvent`s. This can be cloned and passed to game logic.
    pub fn event_sender(&self) -> mpsc::Sender<AudioEvent> {
        self.event_sender.clone()
    }

    /// Loads a sound effect from a file and stores it with a given name.
    pub fn load_sound(&mut self, path: &str, name: &str) -> Result<(), String> {
        let sound_data = StaticSoundData::from_file(Path::new(path), StaticSoundSettings::default())
            .map_err(|e| format!("Failed to load sound '{}': {}", path, e))?;
        self.sounds.insert(name.to_string(), sound_data);
        Ok(())
    }

    /// Plays a loaded sound effect by name.
    pub fn play_sound(&mut self, name: &str) -> Result<(), String> {
        if let Some(sound_data) = self.sounds.get(name) {
            self.manager.play(sound_data.clone())
                .map_err(|e| format!("Failed to play sound '{}': {}", name, e))?;
            Ok(())
        } else {
            Err(format!("Sound '{}' not found", name))
        }
    }

    /// Processes all pending audio events and triggers corresponding sounds.
    pub fn process_events(&mut self) {
        while let Ok(event) = self.event_receiver.try_recv() {
            match event {
                AudioEvent::PlayerJumped => {
                    let _ = self.play_sound("jump"); // TODO: Handle result properly
                }
                // Handle other events here
            }
        }
    }
}
