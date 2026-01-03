//! # Concept: Input Mapping
//! 
//! This module is responsible for translating raw hardware signals into 
//! semantic gameplay intentions. It maps keys and buttons to abstract 
//! commands like 'Jump' or 'Move', decoupling the hardware from game logic.

use crate::ecs::world::World;
use crate::ecs::systems::SystemContext;
use crate::input::InputAction;
use crate::ecs::event::CommandJump;
use crate::ecs::component::MovementIntention;

/// A system that maps input state to entity intentions and commands.
pub struct SystemInput;

impl crate::ecs::systems::System<SystemContext<'_>> for SystemInput {
    /// Translates hardware input into movement intentions and jump commands for players.
    fn update(&mut self, world: &mut World, context: &mut SystemContext<'_>) {
        // 1. Identify all entities controlled by the user.
        let player_entities: Vec<_> = world.player_tags.keys().copied().collect();
        
        for entity in player_entities {
            // 2. Check for mortality states to disable control for dead or dying players.
            let is_dead = if let Some(health) = world.healths.get(&entity) {
                health.current == 0
            } else {
                false
            };

            let is_in_dead_state = if let Some(state_comp) = world.state_components.get(&entity) {
                state_comp.state_machine.current_state.as_ref().map(|s| s.get_name()) == Some("DeadState")
            } else {
                false
            };

            if is_dead || is_in_dead_state {
                // Force intention to zero to stop movement upon death.
                world.add_movement_intention(entity, MovementIntention { x: 0.0 });
                continue;
            }

            // 3. Resolve horizontal movement intentions from the current input state.
            let mut move_dir = 0.0;
            if context.input_state.is_action_pressed(InputAction::MoveLeft) {
                move_dir -= 1.0;
            }
            if context.input_state.is_action_pressed(InputAction::MoveRight) {
                move_dir += 1.0;
            }

            // 4. Update the entity's movement intention component.
            world.add_movement_intention(entity, MovementIntention { x: move_dir });

            // 5. Publish a 'Jump Command' intent if the jump action was triggered this frame.
            if context.input_state.is_action_just_pressed(InputAction::Jump) {
                world.event_bus.publish(CommandJump { entity });
            }
        }
    }
}