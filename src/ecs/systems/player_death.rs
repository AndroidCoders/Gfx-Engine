//! This system detects when a player's health has reached zero and publishes a `PlayerDiedEvent`.

use crate::ecs::event::PlayerDiedEvent;
use crate::ecs::systems::{System, SystemContext};
use crate::ecs::world::World;

/// The system responsible for detecting player death and publishing an event.
pub struct PlayerDeathSystem;

impl System<SystemContext<'_>> for PlayerDeathSystem {
    /// Checks if any player entity's health has reached zero.
    ///
    /// If a player's health is 0 and they are not already in a `DyingState`
    /// (to prevent publishing the event multiple times), this system publishes
    /// a `PlayerDiedEvent`.
    fn update(&mut self, world: &mut World, _context: &mut SystemContext) {
        let mut to_die = Vec::new();
        for (entity, health) in &world.healths {
            if health.current == 0 {
                // Check if the entity is a player and not already in DyingState
                if world.player_tags.contains_key(entity) {
                    if let Some(state_comp) = world.state_components.get(entity) {
                        if state_comp.state_machine.current_state.as_ref().is_some_and(|s| s.get_name() != "DyingState") {
                            to_die.push(*entity);
                        }
                    } else {
                        // If there's no state component, they can't be in DyingState, so they should die.
                        to_die.push(*entity);
                    }
                }
            }
        }

        for entity in to_die {
            world.event_bus.publish(PlayerDiedEvent { player: entity });
        }
    }
}
