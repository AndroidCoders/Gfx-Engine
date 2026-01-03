//! # Synchronization: Level Transition
//! 
//! This module orchestrates the sequence of events required to move between levels.
//! it monitors goal-reaching facts and manages the transition state machine,
//! bridging gameplay logic with the level loading engine.

use crate::ecs::systems::{System, SystemContext};
use crate::ecs::event::{EventStartTransition, TransitionType};

/// A system that monitors goal collisions and triggers the level transition sequence.
pub struct SystemWorldLevelTransition {
    pending_level: Option<String>,
}

impl SystemWorldLevelTransition {
    pub fn new() -> Self {
        Self {
            pending_level: None,
        }
    }
}

impl Default for SystemWorldLevelTransition {
    fn default() -> Self {
        Self::new()
    }
}

impl System<SystemContext<'_>> for SystemWorldLevelTransition {
    /// Monitors goal proximity and coordinates the visual/auditory transition sequence.
    fn update(&mut self, world: &mut crate::ecs::world::World, context: &mut SystemContext<'_>) {
        // 1. Check if a previously triggered visual transition has completed.
        if world.transition_finished
            && let Some(level_path) = &self.pending_level {
                // If complete, signal the GameStateManager to load the new level data.
                println!("[LevelTransition] Transition Complete flag detected! Switching to level: {}", level_path);
                *context.next_level = Some(level_path.clone());
                self.pending_level = None;
                world.transition_finished = false;
                return; 
            }

        // 2. Prevent overlapping transition triggers.
        if self.pending_level.is_some() {
            return;
        }

        // 3. Detect physical overlap between the Player and Goal entities.
        let player_entity = world.player_tags.keys().next().copied();

        if let Some(player_entity) = player_entity {
            let goal_entities: Vec<_> = world.goals.keys().copied().collect();

            for &goal_entity in &goal_entities {
                if let (Some(player_collision), Some(goal_collision)) = (world.collisions.get(&player_entity), world.collisions.get(&goal_entity))
                        && player_collision.rect.has_intersection(goal_collision.rect) {
                            
                            // 4. Identify the destination level from the goal's properties.
                            if let Some(next_level_comp) = world.next_levels.get(&goal_entity) {
                                
                                // 5. Orchestrate the start of the sequence: Shutter close + Music fade.
                                println!("[LevelTransition] Goal Reached! Starting transition to {}", next_level_comp.0);
                                self.pending_level = Some(next_level_comp.0.clone());
                                
                                world.event_bus.publish(EventStartTransition {
                                    transition_type: TransitionType::IrisOut,
                                    duration: 1.0,
                                    center: None,
                                });
                                
                                let _ = context.audio_sender.send(crate::audio::AudioEvent::FadeOutMusic(1.0));
                            }
                        }
            }
        }
    }
}