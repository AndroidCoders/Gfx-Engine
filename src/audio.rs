// src/audio.rs

//! Handles all audio loading and playback using the Kira library.
//!
//! This module defines an event-based audio manager, `GameAudioManager`, which
//! decouples audio playback from the game logic. Systems can send `AudioEvent`s
//! to the manager, which then processes them to play the corresponding sounds.

use kira::{
    manager::{AudioManager, AudioManagerSettings, backend::DefaultBackend},
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
};
use std::collections::HashMap;
use std::path::Path;
use std::sync::mpsc;

/// Represents the types of audio events that can be sent to the `GameAudioManager`.
pub enum AudioEvent {
    /// Play a sound effect by its unique string identifier.
    PlaySound(String),
}

/// Manages loading and playing all audio assets for the game.
pub struct GameAudioManager {
    /// The underlying Kira audio manager.
    manager: AudioManager<DefaultBackend>,
    /// A map of loaded sound effects, indexed by name.
    sounds: HashMap<String, StaticSoundData>,
    /// The receiver for incoming audio events from the game systems.
    event_receiver: mpsc::Receiver<AudioEvent>,
    /// The sender for audio events, which can be cloned and passed to systems.
    event_sender: mpsc::Sender<AudioEvent>,
}

impl GameAudioManager {
    /// Creates a new `GameAudioManager`, initializes the audio backend, and loads
    /// all sounds specified in the configuration.
    ///
    /// # Arguments
    ///
    /// * `audio_config` - A map where keys are sound names and values are file paths.
    ///
    /// # Errors
    ///
    /// Returns an error string if the audio manager fails to initialize or if
    /// any of the sound files cannot be loaded.
    pub fn new(audio_config: &HashMap<String, String>) -> Result<Self, String> {
        let manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())
            .map_err(|e| format!("Failed to create Kira audio manager: {}", e))?;

        let (event_sender, event_receiver) = mpsc::channel();

        let mut sounds = HashMap::new();
        for (name, path) in audio_config {
            let sound_data = StaticSoundData::from_file(Path::new(path), StaticSoundSettings::default())
                .map_err(|e| format!("Failed to load sound '{}': {}", path, e))?;
            sounds.insert(name.clone(), sound_data);
        }

        Ok(Self {
            manager,
            sounds,
            event_receiver,
            event_sender,
        })
    }

    /// Returns a clonable sender for `AudioEvent`s.
    ///
    /// This allows any part of the game logic to send audio events to the manager
    /// for processing.
    pub fn event_sender(&self) -> mpsc::Sender<AudioEvent> {
        self.event_sender.clone()
    }

    /// Plays a loaded sound effect by its unique identifier.
    fn play_sound(&mut self, name: &str) -> Result<(), String> {
        if let Some(sound_data) = self.sounds.get(name) {
            self.manager.play(sound_data.clone())
                .map_err(|e| format!("Failed to play sound '{}': {}", name, e))?;
            Ok(())
        } else {
            Err(format!("Sound '{}' not found", name))
        }
    }

    /// Processes all pending audio events in the queue.
    ///
    /// This should be called once per frame to ensure all audio events are handled.
    pub fn process_events(&mut self) {
        while let Ok(event) = self.event_receiver.try_recv() {
            match event {
                AudioEvent::PlaySound(sound_name) => {
                    let _ = self.play_sound(&sound_name); // TODO: Add proper error logging instead of ignoring.
                }
            }
        }
    }
}
