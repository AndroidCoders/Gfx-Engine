//! This system manages high-level game state transitions, such as lives and game over.

use crate::ecs::event::PlayerDiedEvent;
use crate::ecs::systems::{System, SystemContext};
use crate::ecs::world::World;

/// A conductor system that listens for game state events and manages the flow of the game.
pub struct GameFlowSystem;

impl System<SystemContext<'_>> for GameFlowSystem {
    /// Reads events from the event bus and updates the game state accordingly.
    fn update(&mut self, world: &mut World, context: &mut SystemContext) {
        // --- Handle Player Death ---
        for _event in world.event_bus.read::<PlayerDiedEvent>() {
            if *context.lives > 0 {
                *context.lives -= 1;
                println!("[GameFlow] Player died. Lives remaining: {}", *context.lives);
            }

            if *context.lives == 0 {
                println!("[GameFlow] Game Over!");
                // In the future, this will publish a `GameOverEvent`.
            }
        }
    }
}
