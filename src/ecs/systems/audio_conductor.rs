//! This system acts as a "conductor" for audio events.
//!
//! It listens for specific game events published to the event bus and translates
//! them into audio commands, such as playing a sound effect. This decouples the
//! systems that generate game events from the audio system itself.

use crate::ecs::event::CoinCollectedEvent;
use crate::ecs::systems::{System, SystemContext};
use crate::ecs::world::World;
use crate::audio::AudioEvent;

/// The system responsible for handling audio-related events.
pub struct AudioConductorSystem;

impl System<SystemContext<'_>> for AudioConductorSystem {
    /// Reads game events from the event bus and dispatches corresponding audio events.
    ///
    /// For example, it listens for `CoinCollectedEvent` and sends a request to
    /// the `GameAudioManager` to play the coin pickup sound.
    fn update(&mut self, world: &mut World, context: &mut SystemContext) {
        // Read CoinCollectedEvents and play a sound for each one.
        for _ in world.event_bus.read::<CoinCollectedEvent>() {
            if let Some(sound_name) = context.game_config.sound_events.get("coin_pickup") {
                // The actual sound played is determined by the data in game_config.toml
                let _ = context.audio_sender.send(AudioEvent::PlaySound(sound_name.clone()));
            }
        }
    }
}
