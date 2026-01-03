//! # Synchronization: Game Flow
//! 
//! This module orchestrates high-level game consequences based on gameplay facts.
//! It acts as the "Rules Engine" for meta-progression, managing player lives,
//! death sequences, and level-wide state transitions.

use crate::ecs::event::EventPlayerDied;
use crate::ecs::systems::{System, SystemContext};

/// A system that manages lives, respawn timing, and game over triggers.
pub struct SystemGameFlow;

impl System<SystemContext<'_>> for SystemGameFlow {
    /// Responds to player death facts by decrementing lives and scheduling respawns.
    ///
    /// ⚠️ **Hotpath**: Called 120x per second.
    fn update(&mut self, world: &mut crate::ecs::world::World, context: &mut SystemContext<'_>) {
        // 1. Process all 'PlayerDied' facts published in the current frame.
        let events: Vec<EventPlayerDied> = world.event_bus.read::<EventPlayerDied>().cloned().collect();

        for event in events {
            // 2. Only decrement lives if we have a positive balance.
            if world.stats.lives > 0 {
                world.stats.lives -= 1;
                println!("[GameFlow] Player died (Reason: {:?}). Lives remaining: {}", event.reason, world.stats.lives);
                
                // 3. Determine the duration of the respawn sequence based on how the player died.
                // Out-of-bounds falls transition faster than health depletion.
                let duration = if event.reason == crate::ecs::event::PlayerDeathReason::FellOutOfBounds {
                    3.0 
                } else {
                    context.game_config.gameplay.respawn_invincibility_duration
                };

                // 4. Schedule the respawn by adding a RespawnTimer to the entity.
                println!("[GameFlow] Adding RespawnTimer for entity {:?}. Timer: {}", event.player, duration);
                world.add_respawn_timer(event.player, crate::ecs::component::RespawnTimer {
                    timer: duration, 
                    transition_started: false,
                });
            }

            // 5. Trigger the global Game Over state if no lives remain.
            if world.stats.lives == 0 {
                println!("[GameFlow] Game Over!");
                // Future: world.event_bus.publish(EventGameOver);
            }
        }
    }
}