//! # Concept: Entity State Management
//! 
//! This module is the driver for Hierarchical State Machines (HSM). 
//! It advances the logical state of entities (e.g., switching from Idle to Walk)
//! by executing their current state's logic and evaluating transition conditions.

use crate::ecs::systems::{System, SystemContext};

/// A system that advances the state machines for all applicable entities.
pub struct SystemStateMachine;

impl System<SystemContext<'_>> for SystemStateMachine {
    /// Advances the logic of all active Hierarchical State Machines (HSMs).
    ///
    /// ⚠️ **Hotpath**: Called 120x per second.
    fn update(&mut self, world: &mut crate::ecs::world::World, context: &mut SystemContext<'_>) {
        let entities: Vec<_> = world.state_components.keys().copied().collect();

        for entity in entities {
            // 1. Temporarily extract the state machine to avoid mutable borrow conflicts.
            if let Some(mut state_comp) = world.state_components.remove(&entity) {
                // 2. Execute the `update()` logic of the current state.
                let next_state = state_comp.state_machine.update(entity, world, context);
                
                // 3. If a state transition occurred, handle exit/enter logic.
                if next_state.is_some() {
                    state_comp.state_machine.change_state(next_state, entity, world, context);
                }

                // 4. Return the state machine component to the world.
                world.state_components.insert(entity, state_comp);
            }
        }
    }
}