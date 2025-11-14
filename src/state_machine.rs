//! This module defines the generic traits and structures for a Hierarchical State Machine (HSM).
//!
//! The state machine is used to control the behavior of entities such as the
//! player and enemies, ensuring that their logic is organized into distinct,
//! manageable states (e.g., `Idle`, `Walking`, `Jumping`).

/// The `State` trait defines the behavior of a single state in the state machine.
///
/// Each state is responsible for its own logic, including what happens when it's
/// entered, exited, and updated, as well as the conditions for transitioning
/// to other states.
pub trait State {
    /// Called once when the state machine enters this state.
    fn enter(&mut self);

    /// Called once when the state machine exits this state.
    fn exit(&mut self);

    /// Called on every frame while this state is active.
    /// This is where the primary logic for the state should reside.
    fn update_with_context(&mut self, world: &mut crate::ecs::world::World, context: &mut crate::ecs::systems::SystemContext, entity: crate::ecs::world::Entity);

    /// Called on every frame to check if a transition to a new state should occur.
    ///
    /// If a transition is needed, this method should return `Some(Box<dyn State>)`
    /// with the new state. Otherwise, it should return `None`.
    fn transition_with_context(&mut self, world: &mut crate::ecs::world::World, context: &mut crate::ecs::systems::SystemContext, entity: crate::ecs::world::Entity) -> Option<Box<dyn State>>;

    /// Returns a string slice representing the name of the state, used for debugging.
    fn get_name(&self) -> &str;
}

/// The `StateMachine` struct manages the current state of an entity and handles transitions.
pub struct StateMachine {
    /// The current active state, wrapped in a `Box` to allow for dynamic dispatch.
    pub current_state: Option<Box<dyn State>>,
}

impl StateMachine {
    /// Creates a new `StateMachine` and immediately enters the given `initial_state`.
    pub fn new<S: State + 'static>(initial_state: S) -> Self {
        let mut state_machine = StateMachine { current_state: None };
        state_machine.set_current_state(Box::new(initial_state));
        state_machine
    }

    /// Updates the current state and checks for transitions.
    ///
    /// This method first calls the `update` logic of the current state, then
    /// checks if a transition to a new state is warranted.
    pub fn update_with_context(&mut self, world: &mut crate::ecs::world::World, context: &mut crate::ecs::systems::SystemContext, entity: crate::ecs::world::Entity) {
        if let Some(state) = self.current_state.as_mut() {
            state.update_with_context(world, context, entity);
            if let Some(next_state) = state.transition_with_context(world, context, entity) {
                self.set_current_state(next_state);
            }
        }
    }

    /// Sets the new current state, calling `exit` on the old state and `enter` on the new one.
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
