//! This system is responsible for updating the state machines of entities.

use crate::ecs::component::StateComponent;
use crate::ecs::systems::{System, SystemContext};
use crate::ecs::world::{Entity, World};
use crate::state_machine::StateMachine;

/// The system that updates all state machines in the world.
pub struct StateMachineSystem;
impl System<SystemContext<'_>> for StateMachineSystem {
    /// Iterates through all entities with a `StateComponent`, extracts their
    /// state machines, updates them, and then re-inserts them into the world.
    ///
    /// This process ensures that each entity's behavior is managed by its
    /// state machine, allowing for complex and dynamic behaviors.
    fn update(&mut self, world: &mut World, context: &mut SystemContext) {
        let mut updates: Vec<(Entity, StateMachine)> = Vec::new();

        // Extract StateMachines to update
        // Using `drain()` temporarily removes the components, allowing mutable
        // access to the world within the state machine's update logic.
        for (entity, state_component) in world.state_components.drain() {
            updates.push((entity, state_component.state_machine));
        }

        // Perform updates and re-insert
        for (entity, mut state_machine) in updates {
            state_machine.update_with_context(world, context, entity);
            world.state_components.insert(entity, StateComponent { state_machine });
        }
    }
}
