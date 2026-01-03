//! # Concept: Entity State Management
//! 
//! This module is the driver for Hierarchical State Machines (HSM). 
//! It advances the logical state of entities (e.g., switching from Idle to Walk)
//! by executing their current state's logic and evaluating transition conditions.

use crate::ecs::systems::{System, SystemContext};

/// A system that advances the state machines for all applicable entities.
pub struct SystemStateMachine;

impl System<SystemContext<'_>> for SystemStateMachine {
    /// Updates the internal logical state of entities and handles state transitions.
    fn update(&mut self, world: &mut crate::ecs::world::World, context: &mut SystemContext<'_>) {
         // 1. Identify all entities that possess a State Machine.
         let entity_ids: Vec<_> = world.state_components.keys().copied().collect();
         
         for entity in entity_ids {
             // 2. Skip entities outside the active simulation range.
             if world.is_dormant(entity) { continue; }

             // 3. Process the state update using a removal pattern to satisfy borrow checker requirements.
             if let Some(mut state_comp) = world.state_components.remove(&entity) {
                 // Execute the current state's logic and evaluate potential transitions.
                 state_comp.state_machine.update_with_context(world, context, entity);
                 // Restore the updated component to the world.
                 world.state_components.insert(entity, state_comp);
             }
         }
    }
}