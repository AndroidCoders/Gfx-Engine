//! This system is responsible for handling the immediate effects of a player's death,
//! such as changing their state and animation.

use crate::ecs::event::PlayerDiedEvent;
use crate::ecs::systems::{System, SystemContext};
use crate::ecs::world::World;
use crate::player::states::DyingState;

/// A conductor system that listens for `PlayerDiedEvent` and triggers the death sequence.
pub struct PlayerDeathTransitionSystem;

impl System<SystemContext<'_>> for PlayerDeathTransitionSystem {
    fn update(&mut self, world: &mut World, context: &mut SystemContext) {
        for event in world.event_bus.read::<PlayerDiedEvent>() {
            let entity = event.player;

            // Set the state machine to the DyingState
            if let Some(state_comp) = world.state_components.get_mut(&entity) {
                let death_animation_duration = if let Some(anim_config) = context.game_config.animation.get("explosion") {
                    (anim_config.frame_count * anim_config.frame_duration) as f32 / 60.0
                } else {
                    1.0 // Default duration if animation not found
                };
                state_comp.state_machine = crate::state_machine::StateMachine::new(DyingState { timer: death_animation_duration });
            }

            // Set the death animation
            if let Some(animation) = world.animations.get_mut(&entity) {
                animation.controller.set_animation("explosion");
            }

            // Reset health for the next life
            if let Some(health) = world.healths.get_mut(&entity) {
                health.current = health.max;
            }
        }
    }
}
