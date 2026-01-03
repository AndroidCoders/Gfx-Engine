//! # Concept: Audio Processing
//! 
//! This module acts as the hardware bridge for the global audio engine. 
//! It is responsible for flushing the internal audio request queues and 
//! ensuring that sounds and music are physically processed by the backend.

use crate::ecs::systems::System;

/// A system that triggers the physical playback of all queued audio events.
pub struct SystemAudio;

impl System<crate::audio::GameAudioManager> for SystemAudio {
    /// Commands the global audio manager to execute pending playback and loading tasks.
    fn update(&mut self, _world: &mut crate::ecs::world::World, audio_manager: &mut crate::audio::GameAudioManager) {
        // 1. Process the entire queue of pending audio events (sounds, music, fades).
        audio_manager.process_events();
    }
}