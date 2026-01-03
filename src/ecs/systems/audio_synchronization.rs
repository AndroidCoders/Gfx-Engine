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
    /// Responds to gameplay events by sending play requests to the audio manager.
    fn update(&mut self, world: &mut crate::ecs::world::World, context: &mut SystemContext<'_>) {
        // Retrieve the player's position to calculate distance-based attenuation for SFX.
        let player_pos = world.player_tags.keys().next().copied().and_then(|e| world.positions.get(&e)).map(|p| p.0).unwrap_or(Vector2D::default());
        let max_hearing_distance = context.game_config.gameplay.audio.max_hearing_distance;

        // --- Rules ---

        // Rule: When a coin is collected -> Play the configured collection sound.
        for _ in world.event_bus.read::<EventCoinCollected>() {
            if let Some(sound_name) = context.game_config.sound_events.get("coin_pickup") {
                let _ = context.audio_sender.send(AudioEvent::PlaySound(sound_name.clone(), PlaySoundParams::default()));
            }
        }

        // Rule: When an enemy is stomped -> Play the configured stomp sound.
        for _ in world.event_bus.read::<EventPlayerEnemyStomped>() {
            if let Some(sound_name) = context.game_config.sound_events.get("enemy_stomp") {
                let _ = context.audio_sender.send(AudioEvent::PlaySound(sound_name.clone(), PlaySoundParams::default()));
            }
        }

        // Rule: When player is damaged -> Play the configured hit sound.
        for _ in world.event_bus.read::<EventPlayerDamaged>() {
            if let Some(sound_name) = context.game_config.sound_events.get("player_hit") {
                let _ = context.audio_sender.send(AudioEvent::PlaySound(sound_name.clone(), PlaySoundParams::default()));
            }
        }

        // Rule: When any entity jumps -> Play a jump sound.
        for event in world.event_bus.read::<EventEntityJumped>() {
            if world.player_tags.contains_key(&event.entity) {
                // Players always play at full volume.
                if let Some(sound_name) = context.game_config.sound_events.get("player_jump") {
                    let _ = context.audio_sender.send(AudioEvent::PlaySound(sound_name.clone(), PlaySoundParams::default()));
                }
            } else if world.enemy_tags.contains_key(&event.entity)
                && let Some(sound_name) = context.game_config.sound_events.get("enemy_jump") {
                    // Enemies attenuate based on distance from the player to avoid audio clutter.
                    let entity_pos = world.positions.get(&event.entity).map(|p| p.0).unwrap_or(player_pos);
                    let distance = ((entity_pos.x - player_pos.x).powi(2) + (entity_pos.y - player_pos.y).powi(2)).sqrt();
                    
                    if distance < max_hearing_distance {
                        let volume = (1.0 - (distance / max_hearing_distance) as f64).max(0.0).powf(context.game_config.gameplay.audio.volume_falloff_power as f64);
                        let params = PlaySoundParams { volume };
                        let _ = context.audio_sender.send(AudioEvent::PlaySound(sound_name.clone(), params));
                    }
            }
        }

        // Rule: When a respawn starts -> Restart music to sync energy with the new attempt.
        for _ in world.event_bus.read::<EventRespawnStarted>() {
            if let Some(track) = &context.current_soundtrack {
                let _ = context.audio_sender.send(crate::audio::AudioEvent::PlayMusic(
                    track.clone(),
                    crate::audio::PlaySoundParams::default()
                ));
            }
        }
    }
}