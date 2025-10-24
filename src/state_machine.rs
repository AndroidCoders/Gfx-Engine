//! Defines the generic traits and structures for a Hierarchical State Machine (HSM).

pub trait State {
    fn enter(&mut self);
    fn exit(&mut self);
    fn update_with_context(&mut self, world: &mut crate::ecs::world::World, context: &mut crate::ecs::system::SystemContext, entity: crate::ecs::world::Entity);
    fn transition_with_context(&mut self, world: &mut crate::ecs::world::World, context: &mut crate::ecs::system::SystemContext, entity: crate::ecs::world::Entity) -> Option<Box<dyn State>>;
    fn get_name(&self) -> &str;
}

pub struct StateMachine {
    pub current_state: Option<Box<dyn State>>,
}

impl StateMachine {
    pub fn new<S: State + 'static>(initial_state: S) -> Self {
        let mut state_machine = StateMachine { current_state: None };
        state_machine.set_current_state(Box::new(initial_state));
        state_machine
    }

    pub fn update_with_context(&mut self, world: &mut crate::ecs::world::World, context: &mut crate::ecs::system::SystemContext, entity: crate::ecs::world::Entity) {
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