//! This system is responsible for translating player-specific events and inputs
//! into concrete actions, such as jumping.

use crate::ecs::event::PlayerJumpEvent;
use crate::ecs::systems::{System, SystemContext};
use crate::ecs::world::World;

/// The system that handles player control logic.
pub struct PlayerControlSystem;

impl System<SystemContext<'_>> for PlayerControlSystem {
    /// Processes player-related events to control player actions.
    ///
    /// It listens for events like `PlayerJumpEvent` and modifies the player's
    /// components accordingly (e.g., changing velocity to make the player jump).
    fn update(&mut self, world: &mut World, context: &mut SystemContext) {
        // --- Handle Jump Events ---
        for event in world.event_bus.read::<PlayerJumpEvent>() {
            let is_grounded = world.is_grounded(event.player);
            if is_grounded
                && let Some(velocity) = world.velocities.get_mut(&event.player) {
                    velocity.0.y = -context.config.physics.jump_strength;
                }
        }
    }
}
