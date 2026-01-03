//! # Synchronization: Audio
//!
//! This module acts as the "Ear" of the engine. It listens for semantic 
//! gameplay facts published to the event bus and translates them into 
//! requests for the Audio Engine, handling distance-based volume scaling.

use crate::ecs::event::{EventCoinCollected, EventPlayerEnemyStomped, EventPlayerDamaged, EventEntityJumped, EventRespawnStarted};
use crate::ecs::systems::{System, SystemContext};
use crate::audio::{AudioEvent, PlaySoundParams};
use crate::math::Vector2D;

/// A system that triggers sound effects and music changes based on game facts.
pub struct SystemAudioSynchronization;

impl System<SystemContext<'_>> for SystemAudioSynchronization {
    /// Synchronizes audio playback with gameplay events.
    ///
    /// ⚠️ **Hotpath**: Called 120x per second.
    ///
    /// # Side Effects
    /// * Triggers SFX via `context.audio_sender`.
    fn update(&mut self, world: &mut crate::ecs::world::World, context: &mut SystemContext<'_>) {
        // 1. Handle Jump Sound
        for _ in world.event_bus.read::<EventEntityJumped>() {
            let _ = context.audio_sender.send(AudioEvent::PlaySound("jump".to_string(), PlaySoundParams { volume: 0.6, ..Default::default() }));
        }

        // 2. Handle Coin Collection Sound
        // We use a counter to detect *how many* coins were collected this frame for potential future pitch modulation.
        let coin_count = world.event_bus.read::<EventCoinCollected>().count();
        if coin_count > 0 {
             let _ = context.audio_sender.send(AudioEvent::PlaySound("coin".to_string(), PlaySoundParams { volume: 0.7, ..Default::default() }));
        }