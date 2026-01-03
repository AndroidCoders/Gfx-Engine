//! # Synchronization: Enemy Rhythm
//!
//! This module acts as a "Musical Conductor" for AI. It enforces a global
//! game rhythm by synchronizing enemy actions (like jumping) to the music beat.
//! It ensures variety and prevents audio/visual clutter through deterministic 
//! round-robin scheduling.

use crate::ecs::systems::{System, EnemyRhythmContext};
use crate::ecs::event::{EventEntityJumped, EventMusicBeat};

/// A system that triggers enemy jumps in sync with the music beat map.
pub struct SystemEnemyRhythm {
    /// Tracks the number of beats detected since level start for phrase logic.
    beat_counter: u32,
}

impl SystemEnemyRhythm {
    pub fn new() -> Self {
        Self {
            beat_counter: 0,
        }
    }
}

impl Default for SystemEnemyRhythm {
    fn default() -> Self {
        Self::new()
    }
}

impl System<EnemyRhythmContext<'_>> for SystemEnemyRhythm {
    /// Orchestrates enemy rhythmic behavior based on incoming music beat facts.
    ///
    /// ⚠️ **Hotpath**: Called 120x per second.
    ///
    /// # Side Effects
    /// * Publishes [crate::ecs::event::EventEntityJumped] for the chosen enemy.
    fn update(&mut self, world: &mut crate::ecs::world::World, context: &mut EnemyRhythmContext<'_>) {
        // 1. Listen for the Music Beat fact published by the Audio Conductor.
        if world.event_bus.read::<EventMusicBeat>().next().is_some() {
            self.beat_counter += 1;

            if let Some(config) = context.game_config.enemy_behavior.get("spider") {
                // 2. Determine if the current beat falls within an 'Active Phase' or a 'Rest Phase'.
                let cycle_length = config.active_beats + config.rest_beats;
                if cycle_length == 0 { return; }
                
                let phase_step = self.beat_counter % cycle_length;

                // Check if we are currently in the Active Phase of the phrase.
                if phase_step < config.active_beats {
                    // Check if this specific beat matches the jump frequency (e.g., every 2nd beat).
                    if self.beat_counter.is_multiple_of(config.beats_per_jump) {
                        
                        let camera = context.camera;
                        let viewport_rect = sdl3::rect::Rect::new(
                            camera.position.x as i32,
                            camera.position.y as i32,
                            camera.virtual_width as u32,
                            camera.virtual_height as u32,
                        );

                        // 3. Identify valid candidates: must be Grounded, an Enemy, and Visible.
                        let mut candidates = Vec::new();
                        for entity in world.enemy_tags.keys() {
                            if world.is_grounded(*entity)
                                && let Some(pos) = world.positions.get(entity) {
                                    let entity_rect = sdl3::rect::Rect::new(
                                        pos.0.x as i32,
                                        pos.0.y as i32,
                                        1, 
                                        1,
                                    );
                                    if viewport_rect.has_intersection(entity_rect) {
                                        candidates.push(*entity);
                                    }
                                }
                        }

                        let candidate_count = candidates.len();
                        if candidate_count > 0 {
                            // 4. Deterministically pick one winner to jump this beat.
                            // Sorting ensures the order is consistent across all clients/runs.
                            candidates.sort();

                            let jump_index = self.beat_counter / config.beats_per_jump;
                            let winner_idx = (jump_index % candidate_count as u32) as usize;
                            let winner = candidates[winner_idx];

                            // 5. Apply the jump force to the chosen winner.
                            if let Some(vel) = world.velocities.get_mut(&winner) {
                                vel.0.y = config.jump_strength;
                            }

                            // 6. Publish a Jump fact so the Audio system knows to play a sound.
                            world.event_bus.publish(EventEntityJumped { entity: winner });
                        }
                    }
                }
            }
        }
    }
}
