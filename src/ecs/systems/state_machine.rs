use crate::ecs::component::StateComponent;
use crate::ecs::systems::{System, SystemContext};
use crate::ecs::world::{Entity, World};
use crate::state_machine::StateMachine;

pub struct StateMachineSystem;
impl System<SystemContext<'_>> for StateMachineSystem {
    fn update(&mut self, world: &mut World, context: &mut SystemContext) {
        let mut updates: Vec<(Entity, StateMachine)> = Vec::new();

        // Extract StateMachines to update
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
