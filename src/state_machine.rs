//! This module defines the generic traits and structures for a Hierarchical State Machine (HSM).
//! 
//! The state machine is used to control the behavior of entities such as the player and enemies.

/// The `State` trait defines the behavior of a state in the state machine.
pub trait State {
    /// Called when entering the state.
    fn enter(&mut self);
    /// Called when exiting the state.
    fn exit(&mut self);
    /// Called every frame to update the state.
    fn update_with_context(&mut self, world: &mut crate::ecs::world::World, context: &mut crate::ecs::systems::SystemContext, entity: crate::ecs::world::Entity);
    /// Called every frame to check for transitions to other states.
    fn transition_with_context(&mut self, world: &mut crate::ecs::world::World, context: &mut crate::ecs::systems::SystemContext, entity: crate::ecs::world::Entity) -> Option<Box<dyn State>>;
    /// Returns the name of the state.
    fn get_name(&self) -> &str;
}

/// The `StateMachine` struct manages the current state of an entity.
pub struct StateMachine {
    pub current_state: Option<Box<dyn State>>,
}

impl StateMachine {
    /// Creates a new `StateMachine` with an initial state.
    pub fn new<S: State + 'static>(initial_state: S) -> Self {
        let mut state_machine = StateMachine { current_state: None };
        state_machine.set_current_state(Box::new(initial_state));
        state_machine
    }

    /// Updates the current state of the state machine.
    pub fn update_with_context(&mut self, world: &mut crate::ecs::world::World, context: &mut crate::ecs::systems::SystemContext, entity: crate::ecs::world::Entity) {
        if let Some(state) = self.current_state.as_mut() {
            state.update_with_context(world, context, entity);
            if let Some(next_state) = state.transition_with_context(world, context, entity) {
                self.set_current_state(next_state);
            }
        }
    }

    fn set_current_state(&mut self, new_state: Box<dyn State>) {
        if let Some(mut old_state) = self.current_state.take() {
            old_state.exit();
        }
        self.current_state = Some(new_state);
        if let Some(state) = self.current_state.as_mut() {
            state.enter();
        }
    }
}
