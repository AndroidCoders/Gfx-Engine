//! # Synchronization: Player Death
//! 
//! This module implements the consequences of the [crate::ecs::event::EventPlayerDied].
//! 
//! # Responsibilities
//! * Listens for death events.
//! * Transitions the player's state machine to [crate::player::states::DyingState] or [crate::player::states::DeadState].
//! * Applies death forces (e.g., jump up when injured).
//! * Updates animations.

use crate::ecs::systems::{System, SystemContext};
use crate::ecs::event::{EventPlayerDied, PlayerDeathReason};
use crate::player::states::{DyingState, DeadState};
use crate::state_machine::StateMachine;

pub struct RulePlayerDeath;

impl System<SystemContext<'_>> for RulePlayerDeath {
    fn update(&mut self, world: &mut crate::ecs::world::World, _context: &mut SystemContext<'_>) {
        // 1. Read all PlayerDied events
        let events: Vec<(crate::ecs::world::Entity, PlayerDeathReason)> = world.event_bus.read::<EventPlayerDied>()
            .map(|e| (e.player, e.reason))
            .collect();

        for (entity, reason) in events {
            match reason {
                PlayerDeathReason::HealthDepleted => {
                    // Action: Transition to "Dying" state (animated death)
                    if let Some(state_comp) = world.state_components.get_mut(&entity) {
                        state_comp.state_machine = StateMachine::new(DyingState { timer: 3.0 });
                    }
                    
                    // Action: Apply a small hop upwards
                    if let Some(vel) = world.velocities.get_mut(&entity) {
                        vel.0.y = -450.0;
                        vel.0.x = 0.0;
                    }

                    // Action: Adjust render offset for the injured sprite
                    if let Some(renderable) = world.renderables.get_mut(&entity) {
                        renderable.vertical_offset += 8;
                    }

                    // Action: Set the "injured" animation
                    if let Some(anim) = world.animations.get_mut(&entity) {
                        let direction = world.directions.get(&entity)
                            .map(|d| d.direction)
                            .unwrap_or(crate::ecs::component::Direction::Right);
                        
                        let anim_name = match direction {
                            crate::ecs::component::Direction::Left => "injured_left",
                            crate::ecs::component::Direction::Right => "injured_right",
                        };
                        anim.controller.set_animation(anim_name);
                    }
                },
                PlayerDeathReason::FellOutOfBounds => {
                    // Action: Instant transition to Dead (waiting for respawn)
                    if let Some(state_comp) = world.state_components.get_mut(&entity) {
                        state_comp.state_machine = StateMachine::new(DeadState);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecs::world::World;
    use crate::ecs::event::EventPlayerDied;
    use crate::ecs::component::StateComponent;
    use crate::state_machine::StateMachine;
    use crate::player::states::IdleState;
    use crate::ecs::systems::SystemContext;
    use crate::config::{load_config, load_game_config};
    use crate::level::Level;
    use crate::input::InputState;

    #[test]
    fn test_player_death_transition() {
        let mut world = World::new();
        let entity = world.create_entity();
        world.add_state_component(entity, StateComponent { state_machine: StateMachine::new(IdleState) });

        let mut system = RulePlayerDeath;
        
        // Publish event
        world.event_bus.publish(EventPlayerDied { player: entity, reason: PlayerDeathReason::HealthDepleted });

         // Mock context
        let config = load_config().unwrap();
        let game_config = load_game_config("assets/game_config.toml").unwrap();
        let (audio_sender, _) = std::sync::mpsc::channel();
        let mut camera = crate::camera::Camera::new(0.0, 0.0, 0.1, 480.0, 270.0, 1000.0, 1000.0, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 1000.0, 100.0, 0.1);
        let level = Level::default();
        let input_state = InputState::default();
        let mut next_level = None;
        let mut benchmarker = crate::benchmarker::Benchmarker::new();

        let mut mock_context = SystemContext {
            config: &config,
            game_config: &game_config,
            delta_time: 0.1,
            camera: &mut camera,
            audio_sender: &audio_sender,
            is_paused: false,
            is_attract_mode: false,
            benchmarker: &mut benchmarker,
            level: &level,
            input_state: &input_state,
            next_level: &mut next_level,
            current_soundtrack: None,
        };

        system.update(&mut world, &mut mock_context);

        if let Some(state) = world.state_components.get(&entity) {
             let state_name = state.state_machine.current_state.as_ref().unwrap().get_name();
             assert_eq!(state_name, "DyingState", "Player should transition to DyingState");
        } else {
            panic!("State component missing");
        }
    }
}
